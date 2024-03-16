mod runner;
mod solutions;
mod utilities;
mod parser;
mod handler;

use crate::parser::arguments_parser::ArgumentsParser;
use handler::{arguments_handler::handle_arguments, inputs_handler::handle_inputs};

fn main() {
    let mut arguments_parser = ArgumentsParser::new();
    let mut do_exit = false;

    handle_arguments(&mut arguments_parser);

    while !do_exit {
        do_exit = handle_inputs(&mut arguments_parser);
    }
}
