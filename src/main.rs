#![feature(thread_id_value)]

use custom_logger::env_logger_init;

fn main() {
    env_logger_init("info");
    log::info!("Hello, world!");
}
