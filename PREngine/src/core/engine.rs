use crate::core::window::WindowManager;

pub struct Engine {
    window_manager: WindowManager,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
        }
    }

    pub fn run(mut self) {
        match self.window_manager.create_window("PREngine", 800, 600) {
            Ok(_) => {
                println!("worked")
            }
            Err(e) => println!("Failed to create window: {:?}", e),
        }
    }
}
