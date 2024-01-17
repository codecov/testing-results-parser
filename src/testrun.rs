use std::fmt::Display;

use pyo3::class::basic::CompareOp;
use pyo3::{prelude::*, pyclass};

use crate::helpers::s;

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass]
pub enum Outcome {
    Pass,
    Error,
    Failure,
    Skip,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Outcome::Pass => write!(f, "Pass"),
            Outcome::Failure => write!(f, "Failure"),
            Outcome::Error => write!(f, "Error"),
            Outcome::Skip => write!(f, "Skip"),
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct Testrun {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub duration: f64,
    #[pyo3(get, set)]
    pub outcome: Outcome,
    #[pyo3(get, set)]
    pub testsuite: String,
    #[pyo3(get, set)]
    pub timestamp: String,
    #[pyo3(get, set)]
    pub failure_message: Option<String>,
}

impl Testrun {
    pub fn empty() -> Testrun {
        Testrun {
            name: s(""),
            duration: 0.0,
            outcome: Outcome::Pass,
            testsuite: s(""),
            timestamp: s(""),
            failure_message: None,
        }
    }
}

#[pymethods]
impl Testrun {
    #[new]
    fn new(
        name: String,
        duration: f64,
        outcome: Outcome,
        testsuite: String,
        timestamp: String,
        failure_message: Option<String>,
    ) -> Self {
        Self {
            name,
            duration,
            outcome,
            testsuite,
            failure_message,
            timestamp,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "({}, {}, {}, {}, {}, {:?})",
            self.name,
            self.outcome,
            self.duration,
            self.testsuite,
            self.timestamp,
            self.failure_message,
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.name == other.name
                && self.outcome == other.outcome
                && self.duration == other.duration
                && self.testsuite == other.testsuite
                && self.timestamp == other.timestamp
                && self.failure_message == other.failure_message),
            _ => todo!(),
        }
    }
}
