use std::{error::Error, io};

use crossterm::event::KeyCode;

use crate::Model;

#[derive(PartialEq)]

// Simple text input messages
pub enum LowLevelMessage {
    Char(char),
    Push,
    Delete,
    Quit,
}

// Higher level command messages
pub enum HighLevelMessage {
    Add(Vec<String>),
    Log(Vec<String>),
    Remove(Vec<String>),
    Complete(Vec<String>),
    Graph(GraphSpecifier),
    Analysis(AnalysisSpecifier),
}

// Specifiers for Graphing
pub enum GraphSpecifier {
    Bar(Vec<String>),
    Line(Vec<String>),
    Scatter(Vec<String>),
}

// Specifiers for Analysis
pub enum AnalysisSpecifier {
    PCA(Vec<String>),
    // More to come
}

// Parsing Trait
pub trait Parseable<T>: Sized {
    fn parse(a: T) -> Option<Self>;
}

// Executable Trait

pub trait Executable {
    fn execute(&self, model: &mut Model) -> Result<(),&str>;
}

// Implement parsing of text input commands
impl Parseable<KeyCode> for LowLevelMessage {
    fn parse(a: KeyCode) -> Option<Self> {
        match a {
            KeyCode::Char('q') => Some(LowLevelMessage::Quit),
            KeyCode::Char(c) => Some(LowLevelMessage::Char(c)),
            KeyCode::Backspace => Some(LowLevelMessage::Delete),
            KeyCode::Enter => Some(LowLevelMessage::Push),
            _ => None,
        }
    }
}

// Implement parsing of high-level commands
impl Parseable<String> for HighLevelMessage {
    fn parse(a: String) -> Option<Self> {
        let parts: Vec<&str> = a.split_whitespace().collect();

        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "add" => Some(HighLevelMessage::Add(collect_args(&parts[1..]))),
            "complete" => Some(HighLevelMessage::Complete(collect_args(&parts[1..]))),
            "remove" => Some(HighLevelMessage::Remove(collect_args(&parts[1..]))),
            _ => if parts.len() < 2 {None} else {
                match parts[0] {
                    "graph" => {
                        match parts[1] {
                            "bar" => Some(HighLevelMessage::Graph(
                                GraphSpecifier::Bar(collect_args(&parts[2..]))
                            )),
                            "line" => Some(HighLevelMessage::Graph(
                                GraphSpecifier::Line(collect_args(&parts[2..]))
                            )),
                            "scatter" => Some(HighLevelMessage::Graph(
                                GraphSpecifier::Scatter(collect_args(&parts[2..]))
                            )),
                            _ => None,
                        }
                    },
                    "analysis" => {
                        match parts[1] {
                            "pca" => Some(HighLevelMessage::Analysis(
                                AnalysisSpecifier::PCA(collect_args(&parts[2..]))
                            )),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            }
        }
    }
}


// Helper method
fn collect_args(stringy: &[&str]) -> Vec<String> {
    stringy.iter().map(|s| s.to_string()).collect()
}

impl Executable for HighLevelMessage {
    fn execute(&self, model: &mut Model) -> Result<(), &str> {
        match self {
            HighLevelMessage::Add(vars) => {
                let task = vars[..vars.len()-1].join(" ");
                if let Some(points) = vars.last() {
                    let points: u8 = points.trim().parse().map_err(|_| "Invalid use of add command")?;
                    model.tasks.insert(task.clone(),(points,false));
                    Ok(())
                } else {
                    Err("Invalid use of add command")
                }
            },
            HighLevelMessage::Remove(vars) => {
                let task = vars[..vars.len()].join(" ");
                let result = model.tasks.remove(&task);
                match result {
                    Some(key) => Ok(()),
                    _ => Err("Key not found")
                }
            },
            HighLevelMessage::Complete(vars) => {
                let marked = vars[..vars.len()].join(" "); 
                if model.tasks.contains_key(&marked){
                    let mut tmp = model.tasks.get(&marked).unwrap().clone();

                    if tmp.1 {
                        Err("Task already completed")
                    } else {
                        tmp.1 = true;
                        model.tasks.insert(marked.clone(),tmp);
                        Ok(())
                    }
                } else {
                    Err("Cannot mark nonexistent task as complete")
                }
            }
            _ => Err("Not implemented yet"),
        }
    }
}