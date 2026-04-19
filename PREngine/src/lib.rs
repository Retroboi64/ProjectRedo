mod core;

use core::engine::Engine;

#[unsafe(no_mangle)]
pub extern "C" fn test() {
    println!("Hello!! From PR_Engine");
}

// We are not making Engine Instances yet just to make starting devlopment simpler
#[unsafe(no_mangle)]
pub extern "C" fn start() {
    let engine = Engine::new();
    engine.run();
}
