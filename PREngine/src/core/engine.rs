use crate::core::{
    error::EngineError,
    window::{WindowDescriptor, WindowManager},
};
use probably_fine_log::{debug, error, info, warn};
use windowed::{ControlFlow, Event, Key};

pub struct Engine {
    window_manager: WindowManager,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
        }
    }

    fn setup(&mut self) {
        self.window_manager.register(WindowDescriptor::default());
        debug!("Engine setup complete");
    }

    pub fn run(mut self) -> Result<(), EngineError> {
        info!("Engine starting");
        self.setup();

        self.window_manager
            .run_primary(|event, _window| match event {
                Event::CloseRequested => ControlFlow::Exit,
                Event::KeyDown(Key::Escape) => ControlFlow::Exit,
                _ => ControlFlow::Continue,
            })?;

        info!("Engine shut down cleanly");
        Ok(())
    }
}
