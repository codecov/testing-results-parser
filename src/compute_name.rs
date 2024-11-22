use crate::testrun::Framework;
use pyo3::prelude::*;
use quick_xml::escape::unescape;
use std::borrow::Cow;

fn compute_pytest(classname: &str, name: &str, filename: &str) -> String {
    let path_components = filename.split('/').count();

    let classname_components = classname.split(".");

    let actual_classname = classname_components
        .skip(path_components)
        .collect::<Vec<_>>()
        .join("::");

    format!("{}::{}::{}", filename, actual_classname, name)
}

pub fn unescape_str(s: &str) -> Cow<'_, str> {
    unescape(s).unwrap_or(Cow::Borrowed(s))
}

#[pyfunction(signature = (classname, name, framework, filename=None))]
pub fn compute_name(
    classname: &str,
    name: &str,
    framework: Framework,
    filename: Option<&str>,
) -> String {
    let name = unescape_str(name);
    let classname = unescape_str(classname);
    let filename = filename.map(|f| unescape_str(f));

    match framework {
        Framework::Jest => name.to_string(),
        Framework::Pytest => {
            if let Some(filename) = filename {
                compute_pytest(&classname, &name, &filename)
            } else {
                format!("{}::{}", classname, name)
            }
        }
        Framework::Vitest => {
            format!("{} > {}", classname, name)
        }
        Framework::PHPUnit => {
            format!("{}::{}", classname, name)
        }
    }
}
