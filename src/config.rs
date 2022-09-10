use clap::Parser;
use crate::color::{Color};

#[derive(Parser)]
#[clap(name = "termatrix")]
pub struct Config {
    #[clap(
        short = 'c', long = "glyph-color",
        env = "TERMATRIX_GLYPH_COLOR",
        help = "primary color of the scrolling glyphs",
        possible_values = &[
            "black", "blue", "cyan", "green", "magenta", "red", "white", "yellow"
        ],
        default_value_t = Color::Green
    )]
    pub color: Color,

    #[clap(
        short = 'f', long = "update-frequency",
        env = "TERMATRIX_UPDATE_FREQUENCY",
        help = "refresh rate (in Hz) for terminal updates",
        default_value_t = 100
    )]
    pub update_frequency: u64,

    #[clap(
        short = 'd', long = "min-step-delay",
        env = "TERMATRIX_UPDATE_FREQUENCY",
        help = "minimum value of random trace delay in ms",
        default_value_t = 25
    )]
    pub min_step_delay: u64,

    #[clap(
        short = 'D', long = "max-step-delay",
        env = "TERMATRIX_UPDATE_FREQUENCY",
        help = "maximum value of random trace delay in ms",
        default_value_t = 75
    )]
    pub max_step_delay: u64,
}

pub fn parse_config() -> Config {
    Config::parse()
}