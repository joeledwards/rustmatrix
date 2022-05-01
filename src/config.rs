use std::env;
use crate::color::{Color, from_name};

pub struct Config {
    pub color: Color,
    pub delay_ms: u64,
}

pub fn parse_config() -> Config {
    let prefix = "TERMATRIX";

    let color = env::var(format!("{}_COLOR", prefix))
        .ok()
        .map(|m| from_name(&m))
        .flatten()
        .unwrap_or(Color::Green);

    let delay_ms = env::var(format!("{}_DELAY_MS", prefix))
        .ok()
        .map(|m| m.parse::<u64>().ok())
        .flatten()
        .unwrap_or(50);

    Config {
        color,
        delay_ms,
    }
}