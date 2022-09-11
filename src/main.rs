mod color;
mod config;
mod event;
mod view;

use crate::event::{Event, Events};
use crate::view::MatrixApp;
use crate::color::ColorPool;
use config::parse_config;

fn main() {
    let config = parse_config();
    let events = Events::new(config.delay_ms);
    let color_pool = ColorPool::new(config.colors.as_str());
    let mut app = MatrixApp::new(color_pool);

    loop {
        match events.next().unwrap() {
            Event::Tick => app.on_tick(),
            Event::Exit => break,
        }
    }
}
