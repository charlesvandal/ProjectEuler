mod runner;
mod solutions;
mod tui;
mod metadata_parser;

fn main() -> std::io::Result<()> {
    let problems = metadata_parser::read_problems_from_file("./src/metadata.json").unwrap();

    let mut terminal = ratatui::init();
    let app_result = tui::App::default().run(&mut terminal, problems);
    ratatui::restore();
    app_result
}
