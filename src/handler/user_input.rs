#[derive(Debug, PartialEq)]
pub enum UserInput {
    SolveAllProblems,
    Exit,
    None
}

impl From<&str> for UserInput {
    fn from(value: &str) -> Self {
        match value  {
            "all" => UserInput::SolveAllProblems,
            "q" => UserInput::Exit,
            _ => UserInput::None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::handler::user_input::UserInput;

    #[test]
    fn test_solve_all_problems_variant_string_conversion () {
        let user_input_all = "all";

        assert_eq!(UserInput::SolveAllProblems, user_input_all.into());
    }

    #[test]
    fn test_exit_variant_string_conversion () {
        let user_input_exit = "q";

        assert_eq!(UserInput::Exit, user_input_exit.into());
    }

    #[test]
    fn test_none_variant_string_conversion () {
        let invalid_user_input_1 = "invalid";
        let invalid_user_input_2 = "d22dsj0";
        let invalid_user_input_3 = "1234567";
        let invalid_user_input_4 = "@";

        assert_eq!(UserInput::None, invalid_user_input_1.into());
        assert_eq!(UserInput::None, invalid_user_input_2.into());
        assert_eq!(UserInput::None, invalid_user_input_3.into());
        assert_eq!(UserInput::None, invalid_user_input_4.into());
    }
}