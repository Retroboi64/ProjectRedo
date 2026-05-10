#![allow(unused_imports)]

mod core;

use core::engine::Engine;

use probably_fine_log::{Level, StderrLogger, set_logger, set_max_level};
use probably_fine_log::{debug, error, info, trace, warn};

use crate::core::engine::EngineManager;

#[unsafe(no_mangle)]
pub extern "C" fn start() {
    set_logger(StderrLogger::new()).unwrap();
    set_max_level(Level::Debug);

    info!("Starting PR_Engine");

    let mut engine = Engine::new();

    engine.create_window("main", 1920, 1080);

    if let Err(e) = engine.run() {
        error!("Engine exited with error: {}", e);
        std::process::exit(1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn CreateEngine() {
    set_logger(StderrLogger::new()).unwrap();
    set_max_level(Level::Debug);

    let mut em = EngineManager::new();
    em.create_engine();
    let e1 = em.get_engine(0);

    e1.create_window("name", 620, 800);
    
    if let Err(e) = e1.run() {
        error!("Engine exited with error: {}", e);
        std::process::exit(1);
    }
}
