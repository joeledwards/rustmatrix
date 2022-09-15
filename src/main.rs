mod color;
mod config;
mod event;
mod view;

use crate::event::{Event, Events};
use crate::view::MatrixApp;
use crate::config::parse_config;
use std::time::Duration;

fn main() {
    let config = parse_config();
    let tick_rate = Duration::from_nanos(1000000000 / config.update_frequency);
    let events = Events::new(tick_rate);
    let mut app = MatrixApp::new(config);

    loop {
        match events.next().unwrap() {
            Event::Tick => app.on_tick(),
            Event::Exit => break,
        }
    }
}
