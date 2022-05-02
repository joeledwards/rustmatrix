use clap::Parser;
use crate::color::{Color};

#[derive(Parser)]
#[clap(name = "termatrix")]
pub struct Config {
    #[clap(
        short = 'c', long = "glyph-color",
        env = "TERMATRIX_GLYPH_COLOR",
        possible_values = &[
            "black", "blue", "cyan", "green", "magenta", "red", "white", "yellow"
        ],
        default_value_t = Color::Green
    )]
    pub color: Color,

    #[clap(
        short = 'd', long = "delay-ms",
        env = "TERMATRIX_DELAY_MS",
        default_value_t = 50
    )]
    pub delay_ms: u64,
}

pub fn parse_config() -> Config {
    Config::parse()
}