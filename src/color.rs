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

pub fn from_name(name: &String) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "blue" => Some(Color::Blue),
        "cyan" => Some(Color::Cyan),
        "green" => Some(Color::Green),
        "magenta" => Some(Color::Magenta),
        "red" => Some(Color::Red),
        "white" => Some(Color::White),
        "yellow" => Some(Color::Yellow),
        _ => None,
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
}
