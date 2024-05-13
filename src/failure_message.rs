use std::collections::{hash_map::Entry, HashMap};

use lazy_static::lazy_static;
use phf::phf_ordered_map;

use pyo3::{class, prelude::*, types::PyString};

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

fn generate_failure_info(failure_message: &Option<String>) -> String {
    match failure_message {
        None => s("No failure message available"),
        Some(x) => {
            let mut resulting_string = x.clone();
            resulting_string = shorten_file_paths(resulting_string);
            resulting_string = escape_failure_message(resulting_string);
            resulting_string
        }
    }
}

#[derive(FromPyObject, Debug)]
pub struct Failure {
    name: String,
    testsuite: String,
    failure_message: Option<String>,
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
    ];
    message.append(&mut details_beginning.to_vec());

    let failures = payload.failures;
    let mut per_testsuite: HashMap<String, Vec<Failure>> = HashMap::new();
    for fail in failures {
        let list_of_failures = match per_testsuite.entry(fail.testsuite.clone()) {
            Entry::Occupied(x) => x.into_mut(),
            Entry::Vacant(x) => x.insert(vec![]),
        };
        list_of_failures.push(fail);
    }

    for (k, v) in per_testsuite.iter().sorted_by_key(|x| x.0) {
        message.push(format!("## Testsuite: {}", k));
        for fail in v {
            let mut split_name = fail.name.split("::");
            let test_name = split_name.next().unwrap();
            let class_name = split_name.next().unwrap();
            message.push(format!("- Test name: {}", test_name));
            message.push(format!("  Class name: {}", class_name));
            message.push(format!(
                "<pre>{}</pre>",
                generate_failure_info(&fail.failure_message)
            ));
        }
    }

    message.push(s("</details>"));

    Ok(&PyString::new(py, &message.join("\n")))
}
