use lazy_static::lazy_static;
use phf::phf_ordered_map;

use pyo3::{exceptions::PyTypeError, prelude::*, types::PyString};

use itertools::Itertools;
use regex::Regex;

use crate::helpers::s;

// Need to use an ordered map to make sure we replace '>' before
// we replace '\n', so that we don't replace the '>' in '<br>'
static REPLACEMENTS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "\"" => "&quot;",
    "'" => "&apos;",
    "<" => "&lt;",
    ">" => "&gt;",
    "&" => "&amp;",
    "\r" =>  "",
    "\n" =>  "<br>",
};

#[pyfunction]
pub fn escape_failure_message(failure_message: String) -> String {
    let mut escaped_failure_message = failure_message.clone();
    for (from, to) in REPLACEMENTS.entries() {
        escaped_failure_message = escaped_failure_message.replace(from, to);
    }
    escaped_failure_message
}

/*
Examples of strings that match:

/path/to/file.txt
/path/to/file
/path/to
path/to:1:2
/path/to/file.txt:1:2

Examples of strings that don't match:

path
file.txt
*/
lazy_static! {
    static ref SHORTEN_PATH_PATTERN: Regex =
        Regex::new(r"(?:\/*[\w\-]+\/)+(?:[\w\.]+)(?::\d+:\d+)*").unwrap();
}

#[pyfunction]
pub fn shorten_file_paths(failure_message: String) -> String {
    let mut resulting_string = failure_message.clone();
    for m in SHORTEN_PATH_PATTERN.find_iter(&failure_message) {
        let filepath = m.as_str();
        let split_file_path: Vec<_> = filepath.split("/").collect();

        if split_file_path.len() > 3 {
            let mut slice = split_file_path.iter().rev().take(3).rev();

            let s = format!("{}{}", ".../", slice.join("/"));
            resulting_string = resulting_string.replace(filepath, &s);
        }
    }
    resulting_string
}

fn generate_test_description(
    testsuite: &String,
    name: &String,
    flags: &Option<Vec<String>>,
    flake: &Option<Flake>,
) -> String {
    let flags_section = match flags {
        Some(flags) => {
            let mut flag_list: Vec<String> = vec![];
            for flag in flags {
                flag_list.push(format!("- {}", flag));
            }

            let flag_list_string = flag_list.join("<br>");
            format!("**Flags**:<br>{}<br>", flag_list_string)
        }
        None => "".to_string(),
    };
    let test_description_section = format!(
        "<pre>Testsuite:<br>{}<br><br>Test name:<br>{}<br>{}</pre>",
        testsuite, name, flags_section
    );

    match flake {
        Some(flake) => {
            let title = match flake.is_new_flake {
                true => "Newly Detected Flake",
                false => "Known Flaky Test",
            };
            format!(
                ":snowflake::card_index_dividers: **{}**<br>{}",
                title, test_description_section
            )
        }
        None => test_description_section,
    }
}

fn generate_failure_info(failure_message: &Option<String>, flake: &Option<Flake>) -> String {
    let failure_message = match failure_message {
        None => s("No failure message available"),
        Some(x) => {
            let mut resulting_string = x.clone();
            resulting_string = shorten_file_paths(resulting_string);
            resulting_string = escape_failure_message(resulting_string);
            format!("{}", resulting_string)
        }
    };
    let flake_section = match flake {
        None => "".to_string(),
        Some(flake) => {
            let mut flake_messages: Vec<String> = vec![];
            for symptom in &flake.symptoms {
                let symptom_msg = match symptom {
                    FlakeSymptomType::ConsecutiveDiffOutcomes => {
                        "Differing outcomes on the same commit"
                    }
                    FlakeSymptomType::FailedInDefaultBranch => "Failure on default branch",
                    FlakeSymptomType::UnrelatedMatchingFailures => {
                        "Matching failures on unrelated branches"
                    }
                }
                .to_string();
                let msg = format!(":snowflake: :card_index_dividers: **{}**<br>", symptom_msg);
                flake_messages.push(msg);
            }
            flake_messages.join("")
        }
    };
    format!("{}<pre>{}</pre>", flake_section, failure_message)
}

#[derive(Debug)]
enum FlakeSymptomType {
    FailedInDefaultBranch,
    ConsecutiveDiffOutcomes,
    UnrelatedMatchingFailures,
}

impl FromPyObject<'_> for FlakeSymptomType {
    fn extract<'source>(ob: &'source PyAny) -> PyResult<Self> {
        match ob.get_type().name()?.ends_with("FlakeSymptomType") {
            true => (),
            false => {
                return Err(PyTypeError::new_err(format!(
                    "Type passed is not FlakeSymptomType: {} {}",
                    ob.get_type().name()?,
                    ob.str()?.to_str()?
                )))
            }
        };
        match ob.str()?.to_str()? {
            "FlakeSymptomType.FAILED_IN_DEFAULT_BRANCH" => {
                Ok(FlakeSymptomType::FailedInDefaultBranch)
            }
            "FlakeSymptomType.CONSECUTIVE_DIFF_OUTCOMES" => {
                Ok(FlakeSymptomType::ConsecutiveDiffOutcomes)
            }
            "FlakeSymptomType.UNRELATED_MATCHING_FAILURES" => {
                Ok(FlakeSymptomType::UnrelatedMatchingFailures)
            }
            _ => return Err(PyTypeError::new_err("Type passed is not FlakeSymptomType")),
        }
    }
}

#[derive(FromPyObject, Debug)]
struct Flake {
    symptoms: Vec<FlakeSymptomType>,
    is_new_flake: bool,
}

#[derive(Debug)]
pub struct Failure {
    name: String,
    testsuite: String,
    failure_message: Option<String>,
    flags: Option<Vec<String>>,
    flake: Option<Flake>,
}

// want to accept objects that may not have the flags or flake attributes set
impl FromPyObject<'_> for Failure {
    fn extract<'source>(ob: &'source PyAny) -> PyResult<Self> {
        let name = ob.getattr("name")?.extract()?;
        let testsuite = ob.getattr("testsuite")?.extract()?;
        let failure_message = ob.getattr("failure_message")?.extract()?;
        let flags: Option<Vec<String>> = match ob.getattr("flags") {
            Err(_) => None,
            Ok(x) => match x.get_type().name()?.ends_with("NoneType") {
                true => None,
                false => Some(x.extract()?),
            },
        };
        let flake: Option<Flake> = match ob.getattr("flake") {
            Err(_) => None,
            Ok(x) => match x.get_type().name()?.ends_with("NoneType") {
                true => None,
                false => Some(x.extract()?),
            },
        };
        Ok(Failure {
            name: name,
            testsuite: testsuite,
            failure_message: failure_message,
            flags: flags,
            flake: flake,
        })
    }
}

#[derive(FromPyObject, Debug)]
pub struct MessagePayload {
    passed: i32,
    failed: i32,
    skipped: i32,
    failures: Vec<Failure>,
}

#[pyfunction]
pub fn build_message<'py>(py: Python<'py>, payload: MessagePayload) -> PyResult<&'py PyString> {
    let mut message: Vec<String> = Vec::new();
    let header = s("### :x: Failed Test Results: ");
    message.push(header);

    let failed: i32 = payload.failed;
    let passed: i32 = payload.passed;
    let skipped: i32 = payload.skipped;

    let completed = failed + passed + skipped;
    let results_summary = format!(
        "Completed {} tests with **`{} failed`**, {} passed and {} skipped.",
        completed, failed, passed, skipped
    );
    message.push(results_summary);
    let details_beginning = [
        s("<details><summary>View the full list of failed tests</summary>"),
        s(""),
        s("| **Test Description** | **Failure message** |"),
        s("| :-- | :-- |"),
    ];
    message.append(&mut details_beginning.to_vec());

    let failures = payload.failures;
    for fail in failures {
        let name = &fail.name;
        let testsuite = &fail.testsuite;
        let flags = &fail.flags;
        let failure_message = &fail.failure_message;
        let flake = &fail.flake;
        let test_description = generate_test_description(testsuite, name, flags, flake);
        let failure_information = generate_failure_info(failure_message, flake);
        let single_test_row = format!("| {} | {} |", test_description, failure_information);
        message.push(single_test_row);
    }

    Ok(&PyString::new(py, &message.join("\n")))
}
