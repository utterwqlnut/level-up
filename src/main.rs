use std::io;
mod messages;
use crate::messages::Messages;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Model::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct Model {
    counter: u8,
    exit: bool,
}

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Get areas for each region
        let first_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(first_split[0]);

        let input_title = Line::from("User Input".bold());
        let input_block = Block::bordered()
            .title(input_title.centered())
            .border_set(border::THICK);
        let input_area = Rect {
            x: first_split[1].x + first_split[1].width / 4,
            y: first_split[1].y,
            width: first_split[1].width / 2,
            height: first_split[1].height,
        };

        let dashboard_area = body_chunks[0];
        let dash_title = Line::from("Dashboard".bold());
        let dash_block = Block::bordered()
            .title(dash_title.centered())
            .border_set(border::THICK);

        let chart_area = body_chunks[1];
        let chart_title = Line::from("Charts".bold());
        let chart_block = Block::bordered()
            .title(chart_title.centered())
            .border_set(border::THICK);

        input_block.render(input_area, buf);
        dash_block.render(dashboard_area, buf);
        chart_block.render(chart_area, buf);
    }
}

impl Model {
    // runs the main loop of the app, MUV framework
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.collect_msg()?;
            match msg {
                Some(msg) => self.update(&msg),
                None => continue,
            }
        }
        Ok(())
    }

    // View portion
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn collect_msg(&mut self) -> io::Result<Option<Messages>> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_event(key_event))
            }
            _ => Ok(None),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Messages> {
        match key_event.code {
            KeyCode::Char('q') => Some(Messages::Quit),
            KeyCode::Left => Some(Messages::Decrement),
            KeyCode::Right => Some(Messages::Increment),
            _ => None,
        }
    }

    // Use different module functions and call setters
    fn update(&mut self, msg: &Messages) {
        match msg {
            Messages::Increment => self.increment_counter(),
            Messages::Decrement => self.decrement_counter(),
            Messages::Quit => self.exit(),
        }
    }

    // Replace with a bunch of setters for "Model" state variables
    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
