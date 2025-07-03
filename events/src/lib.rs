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
        write!(f, "{}", self.content)
    }
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Self::Appointment(t) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: t.truecolor(200, 200, 3).to_string(),
            },
            Self::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".truecolor(0, 255, 0).to_string(),
            },
            Self::Registration(t) => {
                let hours = t.num_seconds() / 3600;
                let minutes = (t.num_seconds() % 3600) / 60;
                let seconds = t.num_seconds() % 60;
                Notification {
                    size: 30,
                    color: (255, 2, 255),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    )
                    .truecolor(255, 2, 255)
                    .to_string(),
                }
            }
            Self::Remainder(t) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: t.truecolor(50, 50, 50).to_string(),
            },
        }
    }
}
