//mod color;

use rand::prelude::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{stdout, Stdout, Write};
use std::mem;
use std::time::{Instant, Duration};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::color::{Color, ColorPool};
use crate::config::{Config};

// TODO: store traces in a TreeSet ordered by next_update_time. The next_update_time is computed
// based on the previous update time and the update_interval of the trace.
// 
// TODO: manage density by tracking how many columns contain traces, and prevent adding new traces
// until density decreases.

enum ColorType {
    White,
    Normal,
}

enum Character {
    Char {
        char: char,
        bold: bool,
        color_type: ColorType,
    },
    Blank,
}

enum GlyphType {
    Eraser,
    Writer { white: bool, rng: ThreadRng },
}

impl GlyphType {
    fn choice_char(&mut self, alphabet: &Vec<char>) -> Character {
        match self {
            GlyphType::Writer { white, ref mut rng } => {
                let char = alphabet.choose(rng).unwrap().to_owned();
                let bold = rng.gen();

                let color_type = if *white {
                    ColorType::White
                } else {
                    ColorType::Normal
                };

                Character::Char {
                    char,
                    bold,
                    color_type,
                }
            }
            GlyphType::Eraser => Character::Blank,
        }
    }
}

struct Glyph {
    node_type: GlyphType,
    y: u16,
    previous_char: Character,
    char: Character,
    alphabet: Vec<char>
}

impl Glyph {
    fn new(mut node_type: GlyphType, alphabet: &Vec<char>) -> Glyph {
        let y = 1;
        let alpha = alphabet.to_vec();
        let char = node_type.choice_char(&alpha);

        Glyph {
            node_type,
            y,
            previous_char: Character::Blank,
            char,
            alphabet: alpha
        }
    }

    fn update(&mut self) {
        self.y += 1;
        let alpha = self.alphabet.to_vec();
        let next_char = self.node_type.choice_char(&alpha);
        self.previous_char = mem::replace(&mut self.char, next_char);
    }
}

struct Trace {
    glyph_count: u16, // The number of glyphs in this trace
    update_delay: Duration, // Minimal delay between trace updates
    last_updated: Instant, // Time of last update
    wait_time: u16,
    rng: ThreadRng,
    glyphs: VecDeque<Glyph>,
    alphabet: Vec<char>,
    is_drawing: bool,
    color: Color,
}

impl Trace {
    fn new(glyph_count: u16, color: Color, min_update_delay: u64, max_update_delay: u64, alphabet: Vec<char>) -> Trace {
        let mut rng = thread_rng();
        let update_delay = Duration::from_millis(rng.gen_range(min_update_delay..max_update_delay));
        let wait_time = rng.gen_range(0..glyph_count);
        let last_updated = Instant::now();
        let alpha = alphabet.to_vec();

        Trace {
            glyph_count,
            update_delay,
            last_updated,
            wait_time,
            rng,
            glyphs: VecDeque::new(),
            alphabet: alpha,
            is_drawing: false,
            color,
        }
    }

    fn spawn_node(&mut self) -> Glyph {
        let max_range = self.glyph_count - 3;
        let start_delay = self.rng.gen_range(1..max_range);
        self.wait_time = start_delay;
        let alpha = self.alphabet.to_vec();

        self.is_drawing = !self.is_drawing;
        if self.is_drawing {
            let white: bool = self.rng.gen();
            Glyph::new(GlyphType::Writer {
                white,
                rng: thread_rng(),
            }, &alpha)
        } else {
            Glyph::new(GlyphType::Eraser, &alpha)
        }
    }

    fn update(&mut self) {
        let now = Instant::now();

        if now.duration_since(self.last_updated).as_millis() >= self.update_delay.as_millis() {
            for glyph in self.glyphs.iter_mut() {
                glyph.update();
            }
            self.last_updated = now;

            if self.wait_time == 0 {
                let glyph = self.spawn_node();
                self.glyphs.push_back(glyph);
            } else {
                self.wait_time -= 1;
            }

            if let Some(glyph) = self.glyphs.front() {
                if glyph.y > self.glyph_count {
                    self.glyphs.pop_front();
                }
            }
        }
    }
}

pub struct MatrixApp {
    columns: Vec<Trace>,
    stdout: RefCell<RawTerminal<Stdout>>,
}

impl MatrixApp {
    pub fn new(config: Config) -> MatrixApp {
        let color_pool = ColorPool::new(config.colors.as_str());
        let (size_x, size_y) = termion::terminal_size().unwrap();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
        let column_count = size_x / 2;
        let minsd = config.min_step_delay;
        let maxsd = config.max_step_delay;
        let alphabet: Vec<char> = config.glyph_set.to_alphabet();

        // TODO: replace the viewport when dimensions change
        // TODO: pass glyph alphabet into the glyph selector

        let columns = (0..column_count).map(|_| Trace::new(size_y, color_pool.random(), minsd, maxsd, alphabet.to_vec())).collect();

        MatrixApp {
            columns,
            stdout: RefCell::new(stdout),
        }
    }

    fn update(&mut self) {
        for column in self.columns.iter_mut() {
            column.update();
        }
    }

    fn draw(&self) {
        for (x, column) in self.columns.iter().enumerate() {
            for glyph in column.glyphs.iter() {
                write!(
                    self.stdout.borrow_mut(),
                    "{}",
                    termion::cursor::Goto((x * 2) as u16, glyph.y)
                )
                .unwrap();

                match &glyph.char {
                    Character::Char {
                        char,
                        bold,
                        color_type,
                    } => {
                        match color_type {
                            ColorType::White => {
                                self.set_white_char_style();
                            }
                            ColorType::Normal => {
                                self.set_normal_char_style(*bold, column.color);
                            }
                        };
                        write!(
                            self.stdout.borrow_mut(),
                            "{}{}",
                            char,
                            termion::style::Reset
                        )
                        .unwrap();
                    }
                    Character::Blank => {
                        write!(self.stdout.borrow_mut(), " ").unwrap();
                    }
                }

                if glyph.y == 1 {
                    continue;
                }

                if let Character::Char {
                    char,
                    bold,
                    color_type: ColorType::White,
                } = &glyph.char
                {
                    self.set_normal_char_style(*bold, column.color);
                    write!(
                        self.stdout.borrow_mut(),
                        "{}{}{}",
                        termion::cursor::Goto((x * 2) as u16, (glyph.y - 1) as u16),
                        char,
                        termion::style::Reset
                    )
                    .unwrap();
                }
            }
        }
        self.stdout.borrow_mut().flush().unwrap();
    }

    fn set_normal_char_style(&self, bold: bool, color: Color) {
        if bold {
            write!(self.stdout.borrow_mut(), "{}", termion::style::Bold,).unwrap();
        }

        write!(
            self.stdout.borrow_mut(),
            "{}",
            termion::color::Fg(color.as_term().as_ref()),
        )
        .unwrap();
    }

    fn set_white_char_style(&self) {
        write!(
            self.stdout.borrow_mut(),
            "{}{}",
            termion::style::Bold,
            termion::color::Fg(termion::color::White)
        )
        .unwrap();
    }

    pub fn on_tick(&mut self) {
        // First we update state (calculate updated traces)
        self.update();

        // Then we update the display
        // (diff with the prior trace states and update the terminal based on the diffs)
        self.draw();
    }
}

impl Drop for MatrixApp {
    fn drop(&mut self) {
        write!(self.stdout.borrow_mut(), "{}", termion::cursor::Show).unwrap();
    }
}
