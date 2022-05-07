#![feature(thread_id_value)]

mod logger;

use logger::env_logger_init;

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");
}
