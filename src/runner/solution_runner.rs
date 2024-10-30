use crate::solutions::s001::S001;
use crate::solutions::s002::S002;
use crate::solutions::s003::S003;
use crate::solutions::solution::{Solution, SolutionResult};
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
            1 => SolutionsID::S001,
            2 => SolutionsID::S002,
            3 => SolutionsID::S003,
            _ => SolutionsID::NumberSolutionsId
        }
    }
}

static SOLUTIONS_RUN_FUNCTION: Lazy<BTreeMap<SolutionsID, SolutionResult>> = Lazy::new(|| {
    let mut hash_map = BTreeMap::new();

    hash_map.insert(SolutionsID::S001, S001::run());
    hash_map.insert(SolutionsID::S002, S002::run());
    hash_map.insert(SolutionsID::S003, S003::run());
    hash_map
});

// TODO This just wont work right now
pub fn run_all_solutions() -> i16 {
    let mut number_successful_solutions = 0;

    for solution_id in SOLUTIONS_RUN_FUNCTION.keys() {
        let run_result = run_solution(&solution_id);

        // if run_result == solution_runner_defines::SUCCESS {
        //     number_successful_solutions += 1;
        // }
    }

    number_successful_solutions
}

pub fn run_solution(solution_id: &SolutionsID) -> Option<&SolutionResult> {
    SOLUTIONS_RUN_FUNCTION.get(solution_id)
}


#[cfg(test)]
mod test {

    // #[test]
    // fn test_run_all_solutions() {
    //     assert_eq!(SolutionsID::NumberSolutionsId as i16, run_all_solutions());
    // }
    //
    // #[test]
    // fn test_run_s001() {
    //     assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S001));
    // }
    //
    // #[test]
    // fn test_run_s002() {
    //     assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S002));
    // }
    //
    // #[test]
    // fn test_run_s003() {
    //     assert_eq!(solution_runner_defines::SUCCESS, run_solution(&SolutionsID::S003));
    // }
}