use std::fmt;
use std::time::Duration;

pub trait Solution {
    fn run() -> SolutionResult;
}

#[derive(Debug, Default, Clone)]
pub struct SolutionResult {
    pub execution_time: Duration,
    pub result: String,
}

impl SolutionResult {
    pub fn new(execution_time: Duration, result: String) -> Self {
        SolutionResult { execution_time, result }
    }
}

impl fmt::Display for SolutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let micros = self.execution_time.as_micros();
        write!(
            f,
            "Answer: {}, Execution Time: {}μs, ",
            self.result, micros
        )
    }
}