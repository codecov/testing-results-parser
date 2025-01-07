use pyo3::exceptions::PyException;
use pyo3::prelude::*;

mod compute_name;
mod failure_message;
mod junit;
mod raw_upload;
mod testrun;

pub use testrun::Testrun;

pyo3::create_exception!(test_results_parser, ParserError, PyException);
pyo3::create_exception!(test_results_parser, ComputeNameError, PyException);

/// A Python module implemented in Rust.
#[pymodule]
fn test_results_parser(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("ParserError", py.get_type::<ParserError>())?;

    m.add_function(wrap_pyfunction!(raw_upload::parse_raw_upload, m)?)?;
    m.add_function(wrap_pyfunction!(failure_message::build_message, m)?)?;
    m.add_function(wrap_pyfunction!(failure_message::escape_message, m)?)?;
    m.add_function(wrap_pyfunction!(failure_message::shorten_file_paths, m)?)?;
    Ok(())
}
