use crate::runner::solution_runner::{run_solution, run_all_solutions, SolutionsID};
use crate::parser::arguments_parser::{ArgumentsParser, ParserReturnCode};

pub fn handle_arguments(arguments_parser: &mut ArgumentsParser) {
    let return_code = arguments_parser.parse_arguments();

    match return_code {
        ParserReturnCode::SolveProblem => {
            let problem_number: SolutionsID = arguments_parser.get_problem_number().into();
            run_solution(&problem_number);
        }
        ParserReturnCode::SolveAll => {
            run_all_solutions();
        }
        ParserReturnCode::SolveNone => {
            // Do nothing
        }
    }
}