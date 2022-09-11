use std::fmt::{Display, Formatter, Result as FormatResult};
use std::str::FromStr;

use rand;
use termion;
use termion::color::{Color as TermColor};


#[derive(Copy,Clone)]
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
    pub fn from_char(c: char) -> Result<Color, String> {
        match c {
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

pub struct ColorPool {
    nextIndex: u32,
    colors: Vec<Color>,
}

impl ColorPool {
    pub fn new(char_seq: &str) -> ColorPool {
        let colors = char_seq_to_colors(char_seq);
        let pool = ColorPool {
            nextIndex: 0,
            colors: colors,
        };
        pool
    }

    pub fn next(&mut self) -> Color {
        let index = self.nextIndex as usize;
        let nextColor = self.colors.get(index).unwrap();
        let color_count = self.colors.len() as u32;
        if self.nextIndex >= color_count {
            self.nextIndex = 0;
        } else {
            self.nextIndex += 1;
        }
        *nextColor
    }

    pub fn random(&self) -> Color {
        let color_count = self.colors.len() as u32;
        let index = (rand::random::<f32>() * (color_count as f32)) as usize;
        *self.colors.get(index).unwrap()
    }
}

fn char_seq_to_colors(seq: &str) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    for c in seq.to_lowercase().as_str().chars() {
        match Color::from_char(c) {
            Ok(color) => colors.push(color),
            Err(_) => (),
        }
    }
    colors
}
