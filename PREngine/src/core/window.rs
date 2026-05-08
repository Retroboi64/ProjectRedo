#![allow(dead_code)]

use std::collections::HashMap;

use crate::core::error::{EngineError, StateError};
use probably_fine_log::{debug, error, info, warn};
use windowed::{ControlFlow, Event, Window, WindowConfig};

#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    Active,
    Destroyed,
}

impl std::fmt::Display for WindowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowState::Active => write!(f, "active"),
            WindowState::Destroyed => write!(f, "destroyed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub persist: bool,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: String::from("PREngine"),
            width: 800,
            height: 600,
            persist: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowEntry {
    pub descriptor: WindowDescriptor,
    pub state: WindowState,
}

pub struct WindowManager {
    entries: HashMap<usize, WindowEntry>,
    next_id: usize,
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
            entries: HashMap::new(),
            next_id: 0,
            primary: None,
            is_running: false,
        }
    }

    pub fn register(&mut self, descriptor: WindowDescriptor) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        debug!(
            "Registering window '{}' ({}x{}) persist={}",
            descriptor.title, descriptor.width, descriptor.height, descriptor.persist
        );
        if self.primary.is_none() {
            self.primary = Some(id);
            debug!("Window {} set as primary", id);
        }
        self.entries.insert(
            id,
            WindowEntry {
                descriptor,
                state: WindowState::Active,
            },
        );
        id
    }

    pub fn register_window(&mut self, title: &str, width: u32, height: u32) -> usize {
        self.register(WindowDescriptor {
            title: title.to_string(),
            width,
            height,
            persist: false,
        })
    }

    pub fn register_window_persistent(&mut self, title: &str, width: u32, height: u32) -> usize {
        self.register(WindowDescriptor {
            title: title.to_string(),
            width,
            height,
            persist: true,
        })
    }

    pub fn set_persist(&mut self, id: usize, persist: bool) -> Result<(), EngineError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(StateError::WindowIndexOutOfBounds(id))?;
        entry.descriptor.persist = persist;
        debug!("Window {} persist={}", id, persist);
        Ok(())
    }

    pub fn set_persist_by_name(&mut self, name: &str, persist: bool) -> Result<(), EngineError> {
        let id = self
            .get_id_by_name(name)
            .ok_or_else(|| StateError::WindowNotFound(name.to_string()))?;
        self.set_persist(id, persist)
    }

    pub fn destroy(&mut self, id: usize) -> Result<(), EngineError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(StateError::WindowIndexOutOfBounds(id))?;
        if entry.state == WindowState::Destroyed {
            return Err(StateError::WindowAlreadyDestroyed(id).into());
        }
        entry.state = WindowState::Destroyed;
        debug!("Window {} destroyed", id);
        if self.primary == Some(id) {
            self.primary = self
                .entries
                .iter()
                .filter(|(k, v)| **k != id && v.state == WindowState::Active)
                .map(|(k, _)| *k)
                .min();
            debug!("Primary window reassigned to {:?}", self.primary);
        }
        Ok(())
    }

    pub fn destroy_by_name(&mut self, name: &str) -> Result<(), EngineError> {
        let id = self
            .get_id_by_name(name)
            .ok_or_else(|| StateError::WindowNotFound(name.to_string()))?;
        self.destroy(id)
    }

    pub fn destroy_all(&mut self) {
        for entry in self.entries.values_mut() {
            entry.state = WindowState::Destroyed;
        }
        self.primary = None;
        debug!("All windows destroyed");
    }

    pub fn descriptor(&self, id: usize) -> Result<&WindowDescriptor, EngineError> {
        self.entries
            .get(&id)
            .filter(|e| e.state == WindowState::Active)
            .map(|e| &e.descriptor)
            .ok_or_else(|| StateError::WindowIndexOutOfBounds(id).into())
    }

    fn descriptor_mut(&mut self, id: usize) -> Result<&mut WindowDescriptor, EngineError> {
        self.entries
            .get_mut(&id)
            .filter(|e| e.state == WindowState::Active)
            .map(|e| &mut e.descriptor)
            .ok_or_else(|| StateError::WindowIndexOutOfBounds(id).into())
    }

    pub fn primary_descriptor(&self) -> Result<&WindowDescriptor, EngineError> {
        let id = self.primary.ok_or(StateError::NoWindowsRegistered)?;
        self.descriptor(id)
    }

    pub fn list_windows(&self) -> Vec<(usize, &WindowDescriptor)> {
        let mut list: Vec<_> = self
            .entries
            .iter()
            .filter(|(_, e)| e.state == WindowState::Active)
            .map(|(id, e)| (*id, &e.descriptor))
            .collect();
        list.sort_by_key(|(id, _)| *id);
        list
    }

    pub fn list_all_windows(&self) -> Vec<(usize, &WindowDescriptor, &WindowState)> {
        let mut list: Vec<_> = self
            .entries
            .iter()
            .map(|(id, e)| (*id, &e.descriptor, &e.state))
            .collect();
        list.sort_by_key(|(id, _, _)| *id);
        list
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<usize> {
        self.entries
            .iter()
            .filter(|(_, e)| e.state == WindowState::Active && e.descriptor.title == name)
            .map(|(id, _)| *id)
            .min()
    }

    pub fn get_ids_by_name(&self, name: &str) -> Vec<usize> {
        let mut ids: Vec<_> = self
            .entries
            .iter()
            .filter(|(_, e)| e.state == WindowState::Active && e.descriptor.title == name)
            .map(|(id, _)| *id)
            .collect();
        ids.sort();
        ids
    }

    pub fn get_descriptor_by_name(&self, name: &str) -> Result<&WindowDescriptor, EngineError> {
        let id = self
            .get_id_by_name(name)
            .ok_or_else(|| StateError::WindowNotFound(name.to_string()))?;
        self.descriptor(id)
    }

    pub fn rename_window(&mut self, id: usize, new_title: &str) -> Result<(), EngineError> {
        let desc = self.descriptor_mut(id)?;
        debug!("Renaming window {} '{}' -> '{}'", id, desc.title, new_title);
        desc.title = new_title.to_string();
        Ok(())
    }

    pub fn resize_window(&mut self, id: usize, width: u32, height: u32) -> Result<(), EngineError> {
        let desc = self.descriptor_mut(id)?;
        debug!("Resizing window {} to {}x{}", id, width, height);
        desc.width = width;
        desc.height = height;
        Ok(())
    }

    pub fn set_primary(&mut self, id: usize) -> Result<(), EngineError> {
        let entry = self
            .entries
            .get(&id)
            .ok_or(StateError::WindowIndexOutOfBounds(id))?;
        if entry.state == WindowState::Destroyed {
            return Err(StateError::WindowAlreadyDestroyed(id).into());
        }
        debug!("Primary window {:?} -> {}", self.primary, id);
        self.primary = Some(id);
        Ok(())
    }

    pub fn primary_id(&self) -> Option<usize> {
        self.primary
    }

    pub fn active_count(&self) -> usize {
        self.entries
            .values()
            .filter(|e| e.state == WindowState::Active)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    pub fn window_exists(&self, id: usize) -> bool {
        self.entries
            .get(&id)
            .map_or(false, |e| e.state == WindowState::Active)
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn run_all<F>(&mut self, event_handler: F) -> Result<(), EngineError>
    where
        F: FnMut(Event, &Window) -> ControlFlow,
    {
        if self.is_running {
            warn!("Attempted to start event loops while already running");
            return Err(StateError::AlreadyRunning.into());
        }

        let primary_id = self.primary.ok_or(StateError::NoWindowsRegistered)?;

        let secondary_descs: Vec<WindowDescriptor> = self
            .entries
            .iter()
            .filter(|(id, e)| **id != primary_id && e.state == WindowState::Active)
            .map(|(_, e)| e.descriptor.clone())
            .collect();

        let mut handles = Vec::new();

        for desc in secondary_descs {
            let config = WindowConfig {
                title: desc.title.clone(),
                width: desc.width,
                height: desc.height,
                ..Default::default()
            };
            let persist = desc.persist;
            let title = desc.title.clone();

            let handle = std::thread::spawn(move || match Window::new(config) {
                Ok(mut window) => {
                    info!("Secondary window '{}' opened", title);
                    let result = window.run(move |event, _win| match event {
                        Event::CloseRequested => {
                            if persist {
                                ControlFlow::Continue
                            } else {
                                ControlFlow::Exit
                            }
                        }
                        _ => ControlFlow::Continue,
                    });
                    if let Err(e) = result {
                        error!("Secondary window '{}' error: {:?}", title, e);
                    }
                }
                Err(e) => {
                    error!("Failed to open secondary window '{}': {:?}", title, e);
                }
            });

            handles.push(handle);
        }

        self.is_running = true;
        let result = self.run(primary_id, event_handler);
        self.is_running = false;

        for handle in handles {
            let _ = handle.join();
        }

        result
    }

    pub fn run_primary<F>(&mut self, event_handler: F) -> Result<(), EngineError>
    where
        F: FnMut(Event, &Window) -> ControlFlow,
    {
        let id = self.primary.ok_or(StateError::NoWindowsRegistered)?;
        self.run(id, event_handler)
    }

    pub fn run<F>(&mut self, id: usize, event_handler: F) -> Result<(), EngineError>
    where
        F: FnMut(Event, &Window) -> ControlFlow,
    {
        let desc = self.descriptor(id)?.clone();
        info!(
            "Opening primary window '{}' ({}x{})",
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

        window.run(event_handler).map_err(|e| {
            error!(
                "Window '{}' event loop exited with error: {:?}",
                desc.title, e
            );
            EngineError::from(e)
        })
    }
}
