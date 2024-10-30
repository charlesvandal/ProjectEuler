mod runner;
mod solutions;
mod utilities;
mod parser;
mod handler;
mod tui;
mod metadata_parser;

// fn main() {
//     let mut arguments_parser = ArgumentsParser::new();
//     let mut do_exit = false;
//
//     handle_arguments(&mut arguments_parser);
//
//     while !do_exit {
//         do_exit = handle_inputs(&mut arguments_parser);
//     }
// }

fn main() -> std::io::Result<()> {
    let problems = metadata_parser::read_problems_from_file("./src/metadata.json").unwrap();

    let mut terminal = ratatui::init();
    let app_result = tui::App::default().run(&mut terminal, problems);
    ratatui::restore();
    app_result
}
