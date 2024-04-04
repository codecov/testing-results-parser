use lazy_static::lazy_static;
use phf::phf_ordered_map;

use pyo3::prelude::*;

use itertools::Itertools;
use regex::Regex;

// Need to use an ordered map to make sure we replace '>' before
// we replace '\n', so that we don't replace the '>' in '<br>'
static REPLACEMENTS: phf::OrderedMap<&'static str, &'static str> = phf_ordered_map! {
    "\"" => "&quot;",
    "'" => "&apos;",
    "<" => "&lt;",
    ">" => "&gt;",
    "?" => "&amp;",
    "\r" =>  "",
    "\n" =>  "<br>",
};

#[pyfunction]
pub fn escape_failure_message(mut failure_message: String) -> String {
    for (from, to) in REPLACEMENTS.entries() {
        failure_message = failure_message.replace(from, to);
    }
    failure_message
}

lazy_static! {
    static ref SHORTEN_PATH_PATTERN: Regex =
        Regex::new(r"(?:\/*[\w\-]+\/)+(?:[\w\.]+)(?::\d+:\d+)*").unwrap();
}

#[pyfunction]
pub fn shorten_file_paths(mut failure_message: String) -> String {
    let original_failure_message = failure_message.clone();
    for m in SHORTEN_PATH_PATTERN.find_iter(&original_failure_message) {
        let filepath = m.as_str();
        let split_file_path: Vec<_> = filepath.split("/").collect();

        if split_file_path.len() > 3 {
            let mut slice = split_file_path.iter().rev().take(3);

            let s = format!("{}{}", ".../", slice.join("/"));
            failure_message = failure_message.replace(filepath, &s);
        }
    }
    failure_message
}
