mod common;
mod engine;
mod input;
mod math;
mod renderer;
mod util;

use engine::Engine;
use windowed::Key;

fn main() {
    let engine = Engine::init();

    engine.run(|input| {
        // --- Movement (held) ---
        if input.held(Key::W) {
            println!("Moving forward");
        }
        if input.held(Key::S) {
            println!("Moving backward");
        }
        if input.held(Key::A) {
            println!("Strafing left");
        }
        if input.held(Key::D) {
            println!("Strafing right");
        }

        // --- Actions (just pressed – fires once per keypress) ---
        if input.just_pressed(Key::Space) {
            println!("Jump!");
        }
        if input.just_pressed(Key::E) {
            println!("Interact");
        }
        if input.just_pressed(Key::F) {
            println!("Toggle flashlight");
        }
        if input.just_pressed(Key::Tab) {
            println!("Opened inventory");
        }

        // --- On release ---
        if input.just_released(Key::Space) {
            println!("Landed");
        }

        // --- Escape to quit ---
        if input.just_pressed(Key::Escape) {
            println!("Quitting...");
            return false; // signals the engine to stop
        }

        // --- Debug: show every active key this frame ---
        let active: Vec<_> = input.active_keys().collect();
        if !active.is_empty() {
            println!("Active keys: {:?}", active);
        }

        true // keep running
    });
}
