use std::io;
use std::io::Write;
use crate::runner::solution_runner::{run_solution, run_all_solutions, SolutionsID};
use crate::parser::arguments_parser::ArgumentsParser;
use crate::handler::user_input::UserInput;

pub fn handle_inputs(arguments_parser: &mut ArgumentsParser) -> bool{
    let mut user_input= String::new();

    print!("Problem number: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = String::from(user_input.trim());

    if UserInput::Exit == user_input.trim().into() {
        return true
    } else if UserInput::SolveAllProblems == user_input.trim().into() {
        run_all_solutions();
    } else if arguments_parser.set_problem_number(&user_input) {
        let problem_number: SolutionsID = arguments_parser.get_problem_number().into();
        run_solution(&problem_number);
    }

    false
}