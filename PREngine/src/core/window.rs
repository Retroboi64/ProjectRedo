use windowed::*;

// Window manager
pub struct WindowManager {
    windows: Vec<Window>,
    is_running: bool,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            is_running: false,
        }
    }

    pub fn get_windows(&self) -> &[Window] {
        &self.windows
    }

    pub fn get_window(&self, index: usize) -> Option<&Window> {
        self.windows.get(index)
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> windowed::Result<()> {
        let config = WindowConfig {
            title: title.to_string(),
            width,
            height,
            ..Default::default()
        };
        let mut window = Window::new(config)?;

        self.is_running = true;
        window.run(|event, _window| match event {
            Event::CloseRequested => ControlFlow::Exit,
            Event::KeyDown(Key::Escape) => ControlFlow::Exit,
            _ => ControlFlow::Continue,
        })
    }
}
