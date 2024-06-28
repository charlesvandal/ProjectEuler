use std::collections::BTreeMap;
use std::time::Instant;

use lazy_static::lazy_static;

//Solutions import
use crate::solutions::{s001, s002, s003};
use crate::utilities::defines::solution_runner_defines;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
#[allow(dead_code)]
pub enum SolutionsID {
    S001,
    S002,
    S003,
    NumberSolutionsId, //Add new IDs before this value
}

impl From<i8> for SolutionsID {
    fn from(value: i8) -> Self {
        match value {
            1 => SolutionsID::S001,
            2 => SolutionsID::S002,
            3 => SolutionsID::S003,
            _ => SolutionsID::NumberSolutionsId
        }
    }
}

lazy_static! {
    static ref SOLUTIONS_RUN_FUNCTION: BTreeMap<SolutionsID, fn() -> bool> = {
        let mut hash_map = BTreeMap::new();
        
        hash_map.insert(SolutionsID::S001, s001::run as fn() -> bool);
        hash_map.insert(SolutionsID::S002, s002::run as fn() -> bool);
        hash_map.insert(SolutionsID::S003, s003::run as fn() -> bool);
        hash_map
    };
}

pub fn run_all_solutions() -> i16 {
    let mut number_successful_solutions = 0;

    for solution_id in SOLUTIONS_RUN_FUNCTION.keys() {
        let run_result = run_solution(&solution_id);

        if run_result == solution_runner_defines::SUCCESS {
            number_successful_solutions += 1;
        }
    }

    number_successful_solutions
}

pub fn run_solution(solution_id: &SolutionsID) -> bool {
    let run_function = SOLUTIONS_RUN_FUNCTION.get(solution_id);

    return match run_function {
        Some(func) => {
            run_solution_with_time(solution_id, func)
        }
        None => {
            println!("Run function not found for solution {:?}", *solution_id);
            solution_runner_defines::FAIL
        }
    };
}

fn run_solution_with_time(solution_id: &SolutionsID, run_function: &fn() -> bool) -> bool {
    println!("Running solution {:?}...", *solution_id);

    let start = Instant::now();
    let run_result = run_function();
    let end = Instant::now();
    let execution_time = end - start;

    println!("Solution {:?} ran for {:?} and returned {}\n", *solution_id, execution_time, run_result);

    run_result
}

#[cfg(test)]
mod test {
    use crate::runner::solution_runner::{run_all_solutions, run_solution, SolutionsID};
    use crate::utilities::defines::solution_runner_defines;

    #[test]
    fn test_run_all_solutions() {
        assert_eq!(SolutionsID::NumberSolutionsId as i16, run_all_solutions());
    }

    #[test]
    fn test_run_s001() {
        assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S001));
    }

    #[test]
    fn test_run_s002() {
        assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S002));
    }

    #[test]
    fn test_run_s003() {
        assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S003));
    }
}