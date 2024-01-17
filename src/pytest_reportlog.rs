use pyo3::prelude::*;

use crate::{
    helpers::{s, ParserError},
    testrun::{Outcome, Testrun},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum ReportTypeEnum {
    SessionStart,
    SessionFinish,
    WarningMessage,
    TestReport,
    CollectReport,
}

#[derive(Serialize, Deserialize, Debug)]
enum WhenEnum {
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "teardown")]
    Teardown,
    #[serde(rename = "collect")]
    Collect,
}

#[derive(Serialize, Deserialize, Debug)]
enum OutcomeEnum {
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "skipped")]
    Skipped,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location(String, i32, String);

#[derive(Serialize, Deserialize, Debug)]
struct ReprCrash {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LongRepr {
    reprcrash: ReprCrash,
}
#[derive(Serialize, Deserialize, Debug)]
struct PytestLine {
    /// report_type denotes the type of object in the pytest reportlog
    /// Since we only care about parsing test results we will only look at objects that have the "TestReport" report_type
    /// possible values are: "SessionStart", "SessionFinish", "WarningMessage", "TestReport", "CollectReport"
    ///
    /// start and stop are floating point timestamps that denote the start and stop times of the test
    ///
    /// location is a tuple that specifies the location of the test being run
    /// it is of the form: (filepath, line number, test name)
    ///
    /// when specifies to which stage of the test run the line in the reportlog belongs to
    /// possible values are: "setup", "call", "teardown", "collect"
    ///
    /// outcome specifies the outcome of the stage of the test run
    /// to get the result of the test itself we must get the outcome at the "call" stages
    /// possible values are: "passed", "failed", "skipped"
    ///
    /// longrepr is an object that contains many different representations of the failure that
    /// may have occured during a test run
    /// reprcrash is a representation of the failure that contains:
    /// - the filepath of the file where the failure occured
    /// - the line number where the failure occured
    /// - the failure message

    #[serde(rename = "$report_type")]
    report_type: ReportTypeEnum,
    start: Option<f64>,
    stop: Option<f64>,
    location: Option<Location>,
    when: Option<WhenEnum>,
    outcome: Option<OutcomeEnum>,
    longrepr: Option<LongRepr>,
}

#[pyfunction]
pub fn parse_pytest_reportlog(file_bytes: Vec<u8>) -> PyResult<Vec<Testrun>> {
    let mut testruns: Vec<Testrun> = Vec::new();

    let file_string = String::from_utf8_lossy(&file_bytes).into_owned();

    let mut saved_start_time: Option<f64> = None;
    let mut saved_failure_message: Option<String> = None;
    let mut saved_outcome: Option<Outcome> = None;

    let mut lineno = 0;

    let string_lines = file_string.lines();

    for line in string_lines {
        let val: PytestLine = serde_json::from_str(line)
            .map_err(|err| ParserError::new_err(format!("Error parsing json line  {}", err)))?;

        if val.report_type == ReportTypeEnum::TestReport {
            match val.when.ok_or(ParserError::new_err(format!(
                "No when attribute on TestReport on lineno {}",
                lineno
            )))? {
                WhenEnum::Setup => {
                    saved_start_time = Some(val.start.ok_or(ParserError::new_err(format!(
                        "No when attribute on TestReport on lineno {}",
                        lineno
                    )))?);
                }
                WhenEnum::Teardown => {
                    let location = val.location.ok_or(ParserError::new_err(format!(
                        "Error reading location on line number {}",
                        lineno
                    )))?;
                    let name = location.2;
                    let testsuite = location.0;

                    let end_time = val.stop.ok_or(ParserError::new_err(format!(
                        "Error reading stop time on line number {}",
                        lineno
                    )))?;
                    let start_time = saved_start_time.ok_or(ParserError::new_err(format!(
                        "Error reading saved start time on line number {}",
                        lineno
                    )))?;

                    let duration = end_time - start_time;

                    let outcome = saved_outcome.ok_or(ParserError::new_err(format!(
                        "Error reading saved outcome when parsing line {}",
                        lineno,
                    )))?;

                    let failure_message = match outcome {
                        Outcome::Failure => {
                            Some(saved_failure_message.ok_or(ParserError::new_err(format!(
                                "Error reading saved failure message when parsing line {}",
                                lineno,
                            )))?)
                        }
                        _ => None,
                    };

                    let timestamp = start_time.to_string();
                    testruns.push(Testrun {
                        name,
                        testsuite,
                        duration,
                        outcome,
                        failure_message,
                        timestamp,
                    });
                    saved_start_time = None;
                    saved_failure_message = None;
                    saved_outcome = None;
                }
                WhenEnum::Call => {
                    saved_failure_message = Some(match val.longrepr {
                        Some(longrepr) => longrepr.reprcrash.message,
                        None => s(""),
                    });

                    saved_outcome = Some(
                        match val.outcome.ok_or(ParserError::new_err(format!(
                            "Error reading outcome when parsing line {}",
                            lineno,
                        )))? {
                            OutcomeEnum::Passed => Outcome::Pass,
                            OutcomeEnum::Failed => Outcome::Failure,
                            OutcomeEnum::Skipped => Outcome::Skip,
                        },
                    );
                }
                _ => (),
            }
        }
        lineno += 1;
    }

    Ok(testruns)
}
