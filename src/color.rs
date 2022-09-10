use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;

use rand::Rng;

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

    fn from_char(c: char) -> Result<Color, String> {
        match c.to_lowercase() {
            'b' => Ok(Color::Blue),
            'c' => Ok(Color::Cyan),
            'g' => Ok(Color::Green),
            'k' => Ok(Color::Black),
            'm' => Ok(Color::Magenta),
            'r' => Ok(Color::Red),
            'w' => Ok(Color::White),
            'y' => Ok(Color::Yellow),
            _ => Err(format!("Unsupported color character '{}'", c)),
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

struct ColorPool {
    next: u32;
    colors: Vec<Color>;
}

impl ColorPool: Rng {
    pub fn new(char_seq: &str) -> ColorPool {
        let colors = char_seq_to_colors(char_seq);
        ColorPool {
            0,
            colors   
        }
    }

    pub fn next(&self) -> Color {
        let nextColor = colors.get(next);
        if (self.next >= colors.len()) {
            self.next = 0;
        } else {
            self.next += 1;
        }
        nextColor
    }

    pub fn random(&self) -> Color {
        let index = self.gen_range(0..colors.len());
        colors.get(index)
    }
}

fn char_seq_to_colors(seq: &str) -> Vec<Color> {
    let colors: Vec<Color> = Vec::new();
    for c in seq.chars() {
        match Color::from_char(c) {
            Ok(color) => colors.push(color),
        }
    }
    colors
}
