use crate::state::AppState;
use crate::tui::App;

mod runner;
mod solutions;
mod tui;
mod metadata_parser;
mod state;

fn main() -> std::io::Result<()> {
    let state = initialize();
    let app_result = App::new(state).run();
    dispose();

    app_result
}

fn initialize() -> AppState {
    let problems = metadata_parser::read_problems_from_file("./src/metadata.json").unwrap();
    AppState::new(problems)
}

fn dispose() {
    ratatui::restore()
}
