use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
        let colored_content = format!("{}{}", color_code, self.content).normal();
        write!(
            f,
            "({:?}, {}, {}) {:?} (size: {}, {:?})",
            self.position,
            self.size,
            colored_content,
            self.position,
            self.size,
            self.position
        )
    }
}

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(duration) => {
                let formatted_duration = format!(
                    "You have {} left before the registration ends",
                    format_duration(*duration)
                );
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: formatted_duration,
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    format!("{:02}H:{:02}M:{:02}S", hours, minutes, seconds)
}


