use clap::{Arg, Command};

use crate::parser::arguments_parser_data::ArgumentsParserData;

const PROBLEM_NUMBER_ARGUMENT_NAME: &str = "problem";
const SOLVE_ALL_PROBLEMS_ARGUMENT_NAME: &str = "all";

pub enum ParserReturnCode {
    SolveProblem,
    SolveAll,
    SolveNone,
}

pub struct ArgumentsParser {
    data: ArgumentsParserData,
}

impl ArgumentsParser {
    pub fn new() -> Self {
        Self {
            data: ArgumentsParserData::new(0)
        }
    }

    pub fn parse_arguments(&mut self) -> ParserReturnCode {
        let app = Command::new("ProjectEulerSolver")
            .version("1.0")
            .author("Charles-David Lachance & Charles Vandal")
            .about("A solver of ProjectEuler problems")
            .arg(Arg::new(PROBLEM_NUMBER_ARGUMENT_NAME)
                .short('p')
                .long(PROBLEM_NUMBER_ARGUMENT_NAME)
                .required(false)
                .num_args(1)
                .help("Problem to solve with format XXX where X is a digit"))
            .arg(Arg::new(SOLVE_ALL_PROBLEMS_ARGUMENT_NAME)
                .short('a')
                .long(SOLVE_ALL_PROBLEMS_ARGUMENT_NAME)
                .required(false)
                .num_args(0)
                .help("Solve all problems"));

        let matches = app.get_matches();

        if matches.contains_id(PROBLEM_NUMBER_ARGUMENT_NAME) {
            let problem_number = matches.get_one::<String>(PROBLEM_NUMBER_ARGUMENT_NAME);
            if self.set_problem_number(&String::from(problem_number.unwrap())) {
                ParserReturnCode::SolveProblem
            } else {
                ParserReturnCode::SolveNone
            }
        } else if matches.contains_id(SOLVE_ALL_PROBLEMS_ARGUMENT_NAME) {
            ParserReturnCode::SolveAll
        } else {
            ParserReturnCode::SolveNone
        }
    }

    pub fn get_problem_number(&self) -> i8 {
        self.data.get_problem_number()
    }

    pub fn set_problem_number(&mut self, parsed_value: &String) -> bool {
        let problem_number = parsed_value.trim().parse::<i8>();

        match problem_number {
            Ok(num) => {
                self.data.set_problem_number(num);
                true
            }
            Err(_) => {
                self.data.set_problem_number(-1);
                println!("Wrong problem number: {parsed_value}");
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::arguments_parser::ArgumentsParser;

    #[test]
    fn test_get_problem_number() {
        let parser = ArgumentsParser::new();
        let expected_problem_number = 0;
        let actual_problem_number = parser.get_problem_number();

        assert_eq!(actual_problem_number, expected_problem_number);
    }
}
