use clap::Parser;

#[derive(Parser)]
#[clap(name = "termatrix")]
pub struct Config {
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

    #[clap(
        short = 'C', long = "colors",
        env = "TERMATRIX_COLORS",
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
