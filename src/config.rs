use clap::Parser;

#[derive(Parser)]
#[clap(name = "termatrix")]
pub struct Config {
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

    #[clap(
        short = 'c', long = "colors",
        env = "TERMATRIX_COLORS",
        help = "Weighted sequence of colors (repeat color character for greater weight). (valid chars are: b, c, g, k, m, r, w, y",
        default_value = "g"
    )]
    pub colors: String,

    #[clap(
        long = "density",
        env = "TERMATRIX_DENSITY",
        default_value_t = 0.5
    )]
    pub density: f64,
}

pub fn parse_config() -> Config {
    Config::parse()
}
