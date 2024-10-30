use crate::metadata_parser::Problem;
use crate::runner::solution_runner;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::{List, ListItem, ListState, StatefulWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block,
    },
    DefaultTerminal, Frame,
};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    state: AppState,
    problems: Vec<Problem>,
    exit: bool,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    list_state: ListState,
}

impl AppState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        AppState {
            list_state,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, problems: Vec<Problem>) -> io::Result<()> {
        self.problems = problems;
        self.state = AppState::new();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_stateful_widget(self, frame.area(), &mut self.state.clone());
    }

    // TODO Could add mouse event?
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('w') => self.select_prev(),
            KeyCode::Down | KeyCode::Char('s') => self.select_next(),
            KeyCode::Enter | KeyCode::Char('e') => self.run_solution(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn select_next(&mut self) {
        if (self.state.list_state.selected().unwrap() < self.problems.len() - 1) {
            self.state.list_state.select_next();
        }
    }

    fn select_prev(&mut self) {
        self.state.list_state.select_previous();
    }

    // TODO This break the tui since it writes directly to the standard output
    /*
        Will need to update how the state is handled.
        In the implementation of Stateful Widget, I need a custom AppState instead of keeping a ref in the App.
        The AppState will keep a reference of the ListState and also have a something like a DisplayState to know
        what solution was run and what to display.
     */
    fn run_solution(&mut self) {
        let solution_index = self.state.list_state.selected().unwrap() as i8;
        let solution_id = solution_runner::SolutionsID::from(solution_index + 1);
        solution_runner::run_solution(&solution_id);
    }
}

impl StatefulWidget for &App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Title::from(" Project Euler ".bold());
        let instructions = Title::from(Line::from(vec![
            " Select ".into(),
            "<Enter>, <e>".blue().bold(),
            " Previous ".into(),
            "<Up>, <w>".blue().bold(),
            " Next ".into(),
            "<Down>, <s>".blue().bold(),
            " Quit ".into(),
            " <Esc>, <q> ".blue().bold(),
        ]));

        let left_block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);


        let items: Vec<ListItem> = self.problems
            .iter()
            // Probably a better way to format the line
            .map(|problem| ListItem::new(format!("{:<4}  {}", problem.id, problem.name)))
            .collect();

        List::new(items)
            .block(left_block)
            .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::LightYellow))
            .highlight_symbol(">> ")
            .render(area, buf, &mut state.list_state.clone());
    }
}