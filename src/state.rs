use crate::metadata_parser::Problem;
use crate::runner::solution_runner;
use crate::solutions::solution::SolutionResult;
use ratatui::widgets::ListState;

#[derive(Debug, PartialEq)]
enum RunningState {
    Exiting,
    Running,
}

#[derive(Debug)]
pub struct AppState {
    running_state: RunningState,
    pub list_state: ListState,
    pub problems: Vec<Problem>,
    pub solution_result: Option<SolutionResult>,
}

impl AppState {
    pub fn new(problems: Vec<Problem>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        AppState {
            list_state,
            problems,
            running_state: RunningState::Running,
            solution_result: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running_state != RunningState::Exiting
    }

    pub fn exit(&mut self) {
        self.running_state = RunningState::Exiting;
    }

    pub fn select_next(&mut self) {
        if self.list_state.selected().unwrap() < self.problems.len() - 1 {
            self.list_state.select_next();
        }
    }

    pub fn select_prev(&mut self) {
        self.list_state.select_previous();
    }

    pub fn run_solution(&mut self) {
        let solution_index = self.list_state.selected().unwrap() as i8;
        let solution_id = solution_runner::SolutionsID::from(solution_index);
        match solution_runner::run_solution(&solution_id) {
            Some(solution_result) => self.solution_result = Some(solution_result),
            None => self.solution_result = None,
        }
    }
}
