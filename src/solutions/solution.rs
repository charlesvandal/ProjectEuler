use std::fmt;
use std::time::{Duration, Instant};

pub trait Solution {
    fn execute() -> String;

    fn run() -> SolutionResult {
        let start = Instant::now();
        let result = Self::execute();
        let end = Instant::now();
        SolutionResult::new(end - start, result)
    }
}

#[derive(Debug, Default, Clone)]
pub struct SolutionResult {
    pub execution_time: Duration,
    pub result: String,
}

impl SolutionResult {
    pub fn new(execution_time: Duration, result: String) -> Self {
        SolutionResult {
            execution_time,
            result,
        }
    }
}

impl fmt::Display for SolutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Answer: {}, Execution Time: {:?}",
            self.result, self.execution_time
        )
    }
}
