use crate::metadata_parser::Problem;
use crate::runner::solution_runner;
use crate::solutions::solution::SolutionResult;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
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
    solution_result: Option<SolutionResult>,
}

impl AppState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        AppState {
            list_state,
            solution_result: None,
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
        if self.state.list_state.selected().unwrap() < self.problems.len() - 1 {
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
        self.state.solution_result = Some(solution_runner::run_solution(&solution_id).unwrap().clone());
    }
}

impl StatefulWidget for &App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(vertical_chunks[0]);

        let title = Line::from(" Project Euler ".bold()).centered();
        let instructions = Line::from(Line::from(vec![
            " Select ".into(),
            "<Enter>, <e>".blue().bold(),
            " Previous ".into(),
            "<Up>, <w>".blue().bold(),
            " Next ".into(),
            "<Down>, <s>".blue().bold(),
            " Quit ".into(),
            " <Esc>, <q> ".blue().bold(),
        ])).centered();

        let block = Block::bordered()
            .title_top(title)
            .title_bottom(instructions)
            .border_set(border::THICK);


        let items: Vec<ListItem> = self.problems
            .iter()
            // Probably a better way to format the line
            .map(|problem| ListItem::new(format!("{:<4}  {}", problem.id, problem.name)))
            .collect();

        StatefulWidget::render(
            List::new(items)
                .block(block)
                .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::LightYellow))
                .highlight_symbol(">> "), horizontal_chunks[0], buf, &mut state.list_state.clone());


        let solution = match self.state.solution_result.as_ref() {
            Some(solution_result) => solution_result.to_string(),
            None => format!("{}", "Run a solution to see the result")
        };

        let answer_block = Block::bordered()
            .border_set(border::DOUBLE);
        Widget::render(Paragraph::new(solution).block(answer_block), vertical_chunks[1], buf);

        let current_index = self.state.list_state.selected().unwrap();
        let description = self.problems[current_index].description.clone();
        let description_block = Block::bordered()
            .border_set(border::THICK);
        Widget::render(Paragraph::new(description).block(description_block), horizontal_chunks[1], buf);
    }
}