#![allow(unused_imports)]

mod core;

use core::engine::Engine;

use probably_fine_log::{Level, StderrLogger, set_logger, set_max_level};
use probably_fine_log::{debug, error, info, trace, warn};

#[unsafe(no_mangle)]
pub extern "C" fn start() {
    set_logger(StderrLogger::new()).unwrap();
    set_max_level(Level::Debug);

    info!("Starting PR_Engine");

    let engine = Engine::new();

    if let Err(e) = engine.run() {
        error!("Engine exited with error: {}", e);
        std::process::exit(1);
    }
}
