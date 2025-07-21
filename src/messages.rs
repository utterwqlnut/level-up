#[derive(PartialEq)]

pub enum LowLevelMessage {
    Char(char),
    Push,
    Delete,
    Quit,
}
pub enum Messages {
    Increment,
    Decrement,
    Quit,
}

// Implement parsing of commands
