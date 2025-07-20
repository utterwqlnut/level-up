use std::io;
mod messages;
use crate::messages::Messages;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Model::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct Model {
    counter: i8,
    exit: bool,
}

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Model {

    // runs the main loop of the app, MUV framework
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.collect_msg()?;
            match msg {
                Some(msg) => {
                    self.update(&msg)
                }
                None => continue
            }
        }
        Ok(())
    }

    // View portion
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());    
    }

    fn collect_msg(&mut self) -> io::Result<Option<Messages>>{
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_event(key_event))
            }
            _ => Ok(None)
        }     
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<Messages>{
        match key_event.code {
            KeyCode::Char('q') => Some(Messages::Quit),
            KeyCode::Left => Some(Messages::Decrement),
            KeyCode::Right => Some(Messages::Increment),
            _ => None,
        }
    }

    // Use different module functions and call setters
    fn update(&mut self,msg: &Messages) {
        match msg {
            Messages::Increment => self.increment_counter(),
            Messages::Decrement => self.decrement_counter(),
            Messages::Quit => self.exit()
        }
    }
    
    // Replace with a bunch of setters for "Model" state variables
    fn decrement_counter(&mut self) {
        self.counter-=1;
    }

    fn increment_counter(&mut self) {
        self.counter+=1;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}