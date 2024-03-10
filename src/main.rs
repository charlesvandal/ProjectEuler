use std::io;
use std::io::Write;

mod runner;
mod solutions;
mod utilities;
mod parser;

use parser::arguments_parser::{Parser, ReturnCode};
use crate::runner::solution_runner::{run_solution, run_all_solutions, SolutionsID};

fn main() {
    let mut parser = Parser::new();
    let arguments = parser.parse_arguments();

    match arguments {
        ReturnCode::SolveProblem => {
            let problem_number: SolutionsID = parser.get_problem_number().into();
            run_solution(&problem_number);
        }
        ReturnCode::SolveAll => {
            run_all_solutions();
        }
        ReturnCode::SolveNone => {
            // Do nothing
        }
    }

    loop {
        let mut user_input= String::new();

        print!("Problem number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = String::from(user_input.trim());

        if user_input.trim() == "q" {
            break
        } else if parser.set_problem_number(&user_input) {
            let problem_number: SolutionsID = parser.get_problem_number().into();
            run_solution(&problem_number);
        }
    }
}
