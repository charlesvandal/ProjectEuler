use crate::solutions::solution::{Solution, SolutionResult};
use crate::solutions::*;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;

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
            0 => SolutionsID::S001,
            1 => SolutionsID::S002,
            2 => SolutionsID::S003,
            _ => SolutionsID::NumberSolutionsId
        }
    }
}

static SOLUTIONS_RUN_FUNCTION: Lazy<BTreeMap<SolutionsID, Box<fn() -> SolutionResult>>> = Lazy::new(|| {
    let mut hash_map = BTreeMap::new();

    hash_map.insert(SolutionsID::S001, Box::new(S001::run as fn() -> SolutionResult));
    hash_map.insert(SolutionsID::S002, Box::new(S002::run));
    hash_map.insert(SolutionsID::S003, Box::new(S003::run));
    hash_map
});

// TODO This just wont work right now
// pub fn run_all_solutions() -> i16 {
//     let mut number_successful_solutions = 0;
//
//     for solution_id in SOLUTIONS_RUN_FUNCTION.keys() {
//         let run_result = run_solution(&solution_id);
//
//         // if run_result == solution_runner_defines::SUCCESS {
//         //     number_successful_solutions += 1;
//         // }
//     }
//
//     number_successful_solutions
// }

pub fn run_solution(solution_id: &SolutionsID) -> Option<SolutionResult> {
    match SOLUTIONS_RUN_FUNCTION.get(solution_id) {
        Some(run_fn) => Some(run_fn()),
        None => None,
    }
}


#[cfg(test)]
mod test {
    use crate::runner::solution_runner::{run_solution, SolutionsID};
    // #[test]
    // fn test_run_all_solutions() {
    //     assert_eq!(SolutionsID::NumberSolutionsId as i16, run_all_solutions());
    // }

    #[test]
    fn test_run_s001() {
        assert!(run_solution(&SolutionsID::S001).is_some());
    }

    #[test]
    fn test_run_s002() {
        assert!(run_solution(&SolutionsID::S002).is_some());
    }

    #[test]
    fn test_run_s003() {
        assert!(run_solution(&SolutionsID::S003).is_some());
    }
}