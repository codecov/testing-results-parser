use lazy_static::lazy_static;
use phf::phf_ordered_map;

use pyo3::{ffi::PySys_ResetWarnOptions, prelude::*, types::PyString};

use itertools::Itertools;
use regex::Regex;

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
pub fn escape_failure_message_pystring<'py>(
    py: Python<'py>,
    failure_message: &PyString,
) -> &'py PyString {
    let mut escaped_failure_message = failure_message.to_string();
    for (from, to) in REPLACEMENTS.entries() {
        escaped_failure_message = escaped_failure_message.replace(from, to);
    }
    &PyString::new(py, &escaped_failure_message)
}

#[pyfunction]
pub fn escape_failure_message_rust_string(failure_message: String) -> String {
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
pub fn shorten_file_paths_rust_string(failure_message: String) -> String {
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

#[pyfunction]
pub fn shorten_file_paths_pystring<'py>(
    py: Python<'py>,
    python_failure_message: &PyString,
) -> &'py PyString {
    let string_to_read_from = python_failure_message.to_string_lossy();
    let mut resulting_string = python_failure_message.to_string_lossy().into_owned();
    for m in SHORTEN_PATH_PATTERN.find_iter(&string_to_read_from) {
        let filepath = m.as_str();
        let split_file_path: Vec<_> = filepath.split("/").collect();

        if split_file_path.len() > 3 {
            let mut slice = split_file_path.iter().rev().take(3).rev();

            let s = format!("{}{}", ".../", slice.join("/"));
            resulting_string = resulting_string.replace(filepath, &s);
        }
    }
    &PyString::new(py, &resulting_string)
}
