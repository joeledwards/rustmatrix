use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;

use termion;
use termion::color::{Color as TermColor};


pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Color, String> {
        match s.to_lowercase().as_str() {
            "black" => Ok(Color::Black),
            "blue" => Ok(Color::Blue),
            "cyan" => Ok(Color::Cyan),
            "green" => Ok(Color::Green),
            "magenta" => Ok(Color::Magenta),
            "red" => Ok(Color::Red),
            "white" => Ok(Color::White),
            "yellow" => Ok(Color::Yellow),
            _ => Err(format!("Unsupported color '{}'", s)),
        }
    }
}

impl Color {
    pub fn as_term(&self) -> Box<dyn TermColor> {
        let c: Box<dyn TermColor> = match *self {
            Color::Black => Box::new(termion::color::Black),
            Color::Blue => Box::new(termion::color::Blue),
            Color::Cyan => Box::new(termion::color::Cyan),
            Color::Green => Box::new(termion::color::Green),
            Color::Magenta => Box::new(termion::color::Magenta),
            Color::Red => Box::new(termion::color::Red),
            Color::White => Box::new(termion::color::White),
            Color::Yellow => Box::new(termion::color::Yellow),
        };

        c
    }

    fn name(&self) -> &str {
        match self {
            Color::Black => "black",
            Color::Blue => "blue",
            Color::Cyan => "cyan",
            Color::Green => "green",
            Color::Magenta => "magenta",
            Color::Red => "red",
            Color::White => "white",
            Color::Yellow => "yellow",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self.name())
    }
}
