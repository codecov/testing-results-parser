use pyo3::prelude::*;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

use crate::compute_name::{compute_name, unescape_str};
use crate::testrun::{check_testsuites_name, Framework, Outcome, ParsingInfo, Testrun};
use crate::ParserError;

struct RelevantAttrs {
    classname: Option<String>,
    name: Option<String>,
    time: Option<String>,
    file: Option<String>,
}

// from https://gist.github.com/scott-codecov/311c174ecc7de87f7d7c50371c6ef927#file-cobertura-rs-L18-L31
fn get_relevant_attrs(attributes: Attributes) -> PyResult<RelevantAttrs> {
    let mut rel_attrs: RelevantAttrs = RelevantAttrs {
        time: None,
        classname: None,
        name: None,
        file: None,
    };
    for attribute in attributes {
        let attribute = attribute
            .map_err(|e| ParserError::new_err(format!("Error parsing attribute: {}", e)))?;
        let bytes = attribute.value.into_owned();
        let value = String::from_utf8(bytes)?;
        match attribute.key.into_inner() {
            b"time" => rel_attrs.time = Some(value),
            b"classname" => rel_attrs.classname = Some(value),
            b"name" => rel_attrs.name = Some(value),
            b"file" => rel_attrs.file = Some(value),
            _ => {}
        }
    }
    Ok(rel_attrs)
}

fn get_attribute(e: &BytesStart, name: &str) -> PyResult<Option<String>> {
    let attr = if let Some(message) = e
        .try_get_attribute(name)
        .map_err(|e| ParserError::new_err(format!("Error parsing attribute: {}", e)))?
    {
        Some(String::from_utf8(message.value.to_vec())?)
    } else {
        None
    };
    Ok(attr)
}

fn populate(
    rel_attrs: RelevantAttrs,
    testsuite: String,
    testsuite_time: Option<String>,
    framework: Option<Framework>,
) -> PyResult<(Testrun, Option<Framework>)> {
    let classname = rel_attrs.classname.unwrap_or_default();

    let name = rel_attrs
        .name
        .ok_or_else(|| ParserError::new_err("No name found"))?;

    let duration = match rel_attrs.time {
        None => testsuite_time
            .ok_or_else(|| ParserError::new_err("No time/duration found"))?
            .parse()?,
        Some(time_str) => time_str.parse()?,
    };

    let mut t = Testrun {
        name,
        classname,
        duration,
        outcome: Outcome::Pass,
        testsuite,
        failure_message: None,
        filename: rel_attrs.file,
        build_url: None,
        computed_name: None,
    };

    let framework = framework.or_else(|| t.framework());
    if let Some(f) = framework {
        let computed_name = compute_name(&t.classname, &t.name, f, t.filename.as_deref());
        t.computed_name = Some(computed_name);
    };

    Ok((t, framework))
}

#[pyfunction]
pub fn parse_junit_xml(file_bytes: &[u8]) -> PyResult<ParsingInfo> {
    let mut reader = Reader::from_reader(file_bytes);
    reader.config_mut().trim_text(true);

    let mut testruns: Vec<Testrun> = Vec::new();
    let mut saved_testrun: Option<Testrun> = None;

    let mut in_failure: bool = false;

    let mut buf = Vec::new();

    let mut framework: Option<Framework> = None;

    // every time we come across a testsuite element we update this vector:
    // if the testsuite element contains the time attribute append its value to this vec
    // else append a clone of the last value in the vec
    let mut testsuite_time: Vec<Option<String>> = vec![];
    let mut testsuite_names: Vec<Option<String>> = vec![];

    loop {
        let event = reader.read_event_into(&mut buf).map_err(|e| {
            ParserError::new_err(format!(
                "Error parsing XML at position: {} {:?}",
                reader.buffer_position(),
                e
            ))
        })?;
        match event {
            Event::Eof => {
                break;
            }
            Event::Start(e) => match e.name().as_ref() {
                b"testcase" => {
                    let rel_attrs = get_relevant_attrs(e.attributes())?;
                    let (testrun, parsed_framework) = populate(
                        rel_attrs,
                        testsuite_names
                            .iter()
                            .rev()
                            .find_map(|e| e.clone())
                            .or_else(|| Some(String::new()))
                            .unwrap_or_default(),
                        testsuite_time.iter().rev().find_map(|e| e.clone()),
                        framework,
                    )?;
                    saved_testrun = Some(testrun);
                    framework = parsed_framework;
                    println!("testrun: {:?}", saved_testrun);
                }
                b"skipped" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or_else(|| ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Skip;
                }
                b"error" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or_else(|| ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Error;
                }
                b"failure" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or_else(|| ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Failure;

                    testrun.failure_message = get_attribute(&e, "message")?
                        .as_ref()
                        .map(|failure_message| unescape_str(failure_message).to_string());

                    in_failure = true;
                }
                b"testsuite" => {
                    testsuite_names.push(get_attribute(&e, "name")?);
                    testsuite_time.push(get_attribute(&e, "time")?);
                }
                b"testsuites" => {
                    let testsuites_name = get_attribute(&e, "name")?;
                    framework = if let Some(name) = testsuites_name {
                        check_testsuites_name(&name)
                    } else {
                        None
                    };
                }
                _ => {}
            },
            Event::End(e) => match e.name().as_ref() {
                b"testcase" => {
                    let testrun = saved_testrun.ok_or_else(|| {
                        ParserError::new_err(
                            "Met testcase closing tag without first meeting testcase opening tag",
                        )
                    })?;
                    testruns.push(testrun);
                    saved_testrun = None;
                }
                b"failure" => in_failure = false,
                b"testsuite" => {
                    testsuite_time.pop();
                    testsuite_names.pop();
                }
                _ => (),
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"testcase" => {
                    let rel_attrs = get_relevant_attrs(e.attributes())?;
                    let (testrun, parsed_framework) = populate(
                        rel_attrs,
                        testsuite_names
                            .iter()
                            .rev()
                            .find_map(|e| e.clone())
                            .or_else(|| Some(String::new()))
                            .unwrap_or_default(),
                        testsuite_time.iter().rev().find_map(|e| e.clone()),
                        framework,
                    )?;
                    testruns.push(testrun);
                    framework = parsed_framework;
                }
                b"failure" => {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or_else(|| ParserError::new_err("Error accessing saved testrun"))?;
                    testrun.outcome = Outcome::Failure;

                    testrun.failure_message = get_attribute(&e, "message")?
                        .as_ref()
                        .map(|failure_message| unescape_str(failure_message).to_string());
                }
                _ => {}
            },
            Event::Text(x) => {
                if in_failure {
                    let testrun = saved_testrun
                        .as_mut()
                        .ok_or_else(|| ParserError::new_err("Error accessing saved testrun"))?;

                    let mut xml_failure_message = x.into_owned();
                    xml_failure_message.inplace_trim_end();
                    xml_failure_message.inplace_trim_start();

                    testrun.failure_message =
                        Some(String::from_utf8(xml_failure_message.to_vec())?)
                            .as_ref()
                            .map(|failure_message| unescape_str(failure_message).to_string());
                }
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        buf.clear()
    }

    Ok(ParsingInfo {
        framework,
        testruns,
    })
}
