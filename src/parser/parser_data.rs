pub struct ParserData {
    problem_number: i8
}

impl ParserData {
    pub fn new(problem_number: i8) -> Self {
        Self {problem_number}
    }

    pub fn get_problem_number(&self) -> i8 {
        self.problem_number
    }

    pub fn set_problem_number(&mut self, _problem_number: i8) {
        self.problem_number = _problem_number;
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parser_data::ParserData;

    #[test]
    fn test_new_parser_data() {
        let expected_problem_number = 42;
        let parser_data: ParserData = ParserData::new(expected_problem_number);
        let actual_problem_number = parser_data.get_problem_number();

        assert_eq!(expected_problem_number, actual_problem_number);
    }

    #[test]
    fn test_set_problem_number() {
        let expected_problem_number = 42;
        let mut parser_data: ParserData = ParserData::new(0);
        let mut actual_problem_number = parser_data.get_problem_number();

        assert_ne!(expected_problem_number, actual_problem_number);

        parser_data.set_problem_number(expected_problem_number);
        actual_problem_number = parser_data.get_problem_number();

        assert_eq!(expected_problem_number, actual_problem_number);
    }
}