use crate::core::error::{EngineError, StateError};
use probably_fine_log::{debug, error, info, warn};
use windowed::{ControlFlow, Event, Key, Window, WindowConfig};

#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: String::from("PREngine"),
            width: 800,
            height: 600,
        }
    }
}

pub struct WindowManager {
    descriptors: Vec<WindowDescriptor>,
    primary: Option<usize>,
    is_running: bool,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
            primary: None,
            is_running: false,
        }
    }

    pub fn register(&mut self, descriptor: WindowDescriptor) -> usize {
        let id = self.descriptors.len();
        debug!(
            "Registering window '{}' ({}x{})",
            descriptor.title, descriptor.width, descriptor.height
        );
        if self.primary.is_none() {
            self.primary = Some(id);
            debug!("Window {} set as primary", id);
        }
        self.descriptors.push(descriptor);
        id
    }

    pub fn register_window(&mut self, title: &str, width: u32, height: u32) -> usize {
        self.register(WindowDescriptor {
            title: title.to_string(),
            width,
            height,
        })
    }

    pub fn descriptor(&self, id: usize) -> std::result::Result<&WindowDescriptor, EngineError> {
        self.descriptors
            .get(id)
            .ok_or_else(|| StateError::WindowIndexOutOfBounds(id).into())
    }

    pub fn primary_descriptor(&self) -> std::result::Result<&WindowDescriptor, EngineError> {
        let id = self.primary.ok_or(StateError::NoWindowsRegistered)?;
        self.descriptor(id)
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn run_primary<F>(&mut self, event_handler: F) -> std::result::Result<(), EngineError>
    where
        F: FnMut(Event, &Window) -> ControlFlow,
    {
        let id = self.primary.ok_or(StateError::NoWindowsRegistered)?;
        self.run(id, event_handler)
    }

    pub fn run<F>(&mut self, id: usize, event_handler: F) -> std::result::Result<(), EngineError>
    where
        F: FnMut(Event, &Window) -> ControlFlow,
    {
        if self.is_running {
            warn!("Attempted to start a second event loop while already running");
            return Err(StateError::AlreadyRunning.into());
        }

        let desc = self.descriptor(id)?.clone();
        info!(
            "Opening window '{}' ({}x{})",
            desc.title, desc.width, desc.height
        );

        let config = WindowConfig {
            title: desc.title.clone(),
            width: desc.width,
            height: desc.height,
            ..Default::default()
        };

        let mut window = Window::new(config).map_err(|e| {
            error!("Failed to create window '{}': {:?}", desc.title, e);
            EngineError::from(e)
        })?;

        self.is_running = true;
        let result = window.run(event_handler);
        self.is_running = false;

        result.map_err(|e| {
            error!(
                "Window '{}' event loop exited with error: {:?}",
                desc.title, e
            );
            EngineError::from(e)
        })
    }
}
