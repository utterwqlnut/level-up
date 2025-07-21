use std::collections::HashMap;
use std::io;
mod messages;
use crate::messages::{LowLevelMessage, HighLevelMessage};
use crate::messages::Parseable;
use crate::messages::Executable;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal, Frame
};
// TODO
// Implement file saving and loading
// Implement Graph/Chart visualizations
// Make it prettier
// Error popups
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Model::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct Model {
    output: String,
    tasks: HashMap<String,(u8,bool)>,
    input: String,
    error: String,
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

        // Create blocks for each section of ui
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
        let copy = self.input.clone();
        let input_text = Paragraph::new(copy)
            .block(input_block)
            .alignment(Alignment::Center);

        let dashboard_area = body_chunks[0];
        let dash_title = Line::from("Dashboard".bold());
        let dash_block = Block::bordered()
            .title(dash_title.centered())
            .border_set(border::THICK);

        let copy = self.output.clone();
        let dash_text = Paragraph::new(copy)
            .block(dash_block)
            .alignment(Alignment::Center);

        let chart_area = body_chunks[1];
        let chart_title = Line::from("Charts".bold());
        let chart_block = Block::bordered()
            .title(chart_title.centered())
            .border_set(border::THICK);

        // Render blocks
        input_text.render(input_area, buf);
        dash_text.render(dashboard_area, buf);
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

    fn collect_msg(&mut self) -> io::Result<Option<LowLevelMessage>> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_event(key_event))
            }
            _ => Ok(None),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<LowLevelMessage> {
        match key_event.code {
            KeyCode::Esc => Some(LowLevelMessage::Quit),
            KeyCode::Char(c) => Some(LowLevelMessage::Char(c)),
            KeyCode::Backspace => Some(LowLevelMessage::Delete),
            KeyCode::Enter => Some(LowLevelMessage::Push),
            _ => None,
        }
    }

    // Use different module functions and call setters
    fn update(&mut self, msg: &LowLevelMessage) {
        match msg {
            LowLevelMessage::Char(c) => self.add_char(*c),
            LowLevelMessage::Delete => self.pop_char(),
            LowLevelMessage::Quit => self.exit(),
            LowLevelMessage::Push => self.push_msg(),
        }
    }

    // Replace with a bunch of setters for "Model" state variables
    fn add_char(&mut self,c: char) {
        self.input.push(c);
    }

    fn pop_char(&mut self) {
        self.input.pop();
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn push_msg(&mut self) {
        // Replace with a high-level message processor
        let msg = HighLevelMessage::parse(self.input.clone());
        if let Some(true_msg) = msg {
            let res = true_msg.execute(self);
            self.output.clear();
            for key in self.tasks.keys() {
                let inside = self.tasks.get(key).unwrap();
                if inside.1 {
                    self.output.push_str(format!("{key} for {} points 😁\n",inside.0).as_str())
                } else {
                    self.output.push_str(format!("{key} for {} points 😢\n",inside.0).as_str())
                }
            }
            self.input.clear();
            
            match res {
                Ok(_) => self.error=String::new(),
                Err(e) => self.error = String::from(e),
            }
        } else {
            self.error = String::from("Invalid Command");
        }
    }
}
