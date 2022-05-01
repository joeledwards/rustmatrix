mod color;
mod config;
mod event;
mod view;

use crate::event::{Event, Events};
use crate::view::MatrixApp;
use config::parse_config;

fn main() {
    let config = parse_config();
    let events = Events::new(config.delay_ms);
    let mut app = MatrixApp::new(config.color);

    loop {
        match events.next().unwrap() {
            Event::Tick => app.on_tick(),
            Event::Exit => break,
        }
    }
}
