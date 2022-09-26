use clap::Parser;

use crate::alphabet::GlyphSet;

#[derive(Parser)]
#[clap(name = "termatrix")]
pub struct Config {
    #[clap(
        short = 'c', long = "colors",
        env = "TERMATRIX_COLORS",
        help = "Weighted sequence of colors (repeat color character for greater weight). Valid chars are: b, c, g, k, m, r, w, y",
        default_value = "g"
    )]
    pub colors: String,

    #[clap(
        short = 'g', long = "glyph-set",
        env = "TERMATRIX_GLYPH_SET",
        help = "The set of glyphs which should be displayed.",
        possible_values = &[
            "all", "alpha", "alpha_lower", "alpha_upper", "alphanum", "binary",
            "decimal", "hex", "hex_lower", "hex_upper", "math", "set", "special", "symbol"
        ],
        default_value_t = GlyphSet::All
    )]
    pub glyph_set: GlyphSet,

    #[clap(
        short = 'f', long = "update-frequency",
        env = "TERMATRIX_UPDATE_FREQUENCY",
        help = "Refresh rate (in Hz) for terminal updates.",
        default_value_t = 60
    )]
    pub update_frequency: u64,

    #[clap(
        short = 'd', long = "min-step-delay",
        env = "TERMATRIX_MIN_STEP_DELAY",
        help = "Minimum value of trace (column) scroll delay in milliseconds.",
        default_value_t = 40
    )]
    pub min_step_delay: u64,

    #[clap(
        short = 'D', long = "max-step-delay",
        env = "TERMATRIX_MAX_STEP_DELAY",
        help = "Maximum value of trace (column) scroll delay in milliseconds.",
        default_value_t = 120
    )]
    pub max_step_delay: u64,
}

pub fn parse_config() -> Config {
    Config::parse()
}
