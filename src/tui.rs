use crate::state::AppState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, StatefulWidget, Widget, Wrap},
};
use std::io;

#[derive(Debug)]
pub struct App {
    state: AppState,
}

impl App {
    pub fn new(state: AppState) -> Self {
        App { state }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        while self.state.is_running() {
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
            self.handle_events()?;
        }
        Ok(())
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
            KeyCode::Esc | KeyCode::Char('q') => self.state.exit(),
            KeyCode::Up | KeyCode::Char('w') => self.state.select_prev(),
            KeyCode::Down | KeyCode::Char('s') => self.state.select_next(),
            KeyCode::Enter | KeyCode::Char('e') => self.state.run_solution(),
            _ => {}
        }
    }

    fn render_main_window(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Project Euler ").blue().bold().centered();
        let instructions = Line::from(vec![
            " Select ".into(),
            "<Enter>, <e>".blue().bold(),
            " Previous ".into(),
            "<Up>, <w>".blue().bold(),
            " Next ".into(),
            "<Down>, <s>".blue().bold(),
            " Quit ".into(),
            "<Esc>, <q> ".blue().bold(),
        ])
        .centered();

        Block::bordered()
            .title_top(title)
            .title_bottom(instructions)
            .border_set(border::THICK)
            .render(area, buf);
    }

    fn render_result(&self, area: Rect, buf: &mut Buffer) {
        let solution = match self.state.solution_result.as_ref() {
            Some(solution_result) => solution_result.to_string(),
            None => "Run a solution to see the result".to_string(),
        };
        let block = Block::bordered().border_set(border::DOUBLE);

        Paragraph::new(solution).block(block).render(area, buf);
    }

    fn render_problem_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_set(border::THICK);

        let items: Vec<ListItem> = self
            .state
            .problems
            .iter()
            .map(|problem| ListItem::new(format!("{:<4}  {}", problem.id, problem.name)))
            .collect();

        StatefulWidget::render(
            List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::LightYellow))
                .highlight_symbol(">> "),
            area,
            buf,
            &mut self.state.list_state,
        );
    }

    fn render_description(&mut self, area: Rect, buf: &mut Buffer) {
        let current_index = self.state.list_state.selected().unwrap();
        let description = self.state.problems[current_index].description.clone();
        let description_block = Block::bordered().border_set(border::THICK);

        Paragraph::new(description)
            .wrap(Wrap { trim: true })
            .block(description_block)
            .render(area, buf);
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(3)
            .constraints([Constraint::Percentage(80), Constraint::Min(3)])
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(3),
                Constraint::Percentage(60),
            ])
            .split(vertical_chunks[0]);

        self.render_main_window(area, buf);
        self.render_result(vertical_chunks[1], buf);
        self.render_problem_list(horizontal_chunks[0], buf);
        self.render_description(horizontal_chunks[2], buf);
    }
}
