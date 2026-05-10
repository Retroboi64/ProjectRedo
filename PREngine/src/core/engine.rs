#![allow(dead_code)]

use std::process::id;

use crate::core::{
    error::{EngineError, StateError},
    window::{WindowDescriptor, WindowManager, WindowState},
};
use probably_fine_log::{debug, error, info, warn};
use windowed::{ControlFlow, Event, Key};

#[derive(Debug, Clone, PartialEq)]
pub enum EngineLifecycle {
    Created,
    Running,
    Paused,
    Shutdown,
}

impl std::fmt::Display for EngineLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineLifecycle::Created => write!(f, "created"),
            EngineLifecycle::Running => write!(f, "running"),
            EngineLifecycle::Paused => write!(f, "paused"),
            EngineLifecycle::Shutdown => write!(f, "shutdown"),
        }
    }
}

pub struct EngineManager {
    pub engines: Vec<Engine>,
    pub current: usize,
}

impl EngineManager {
    pub fn new() -> Self {
        Self {
            engines: Vec::new(),
            current: 0,
        }
    }

    pub fn create_engine(&mut self) -> usize {
        let id = self.current + 1;
        info!("Creating engine: {}", id);

        self.engines.push(Engine::new());
        self.current = id;

        id
    }

    pub fn get_engine(&mut self, id: usize) -> &mut Engine {
        info!("Getting engine: {}", id);
        &mut self.engines[id]
    }

    pub fn get_current_engine(&mut self) -> &mut Engine {
        let id = self.current;

        info!("Getting current: {}", id);
        &mut self.engines[id]
    }
}

pub struct Engine {
    window_manager: WindowManager,
    lifecycle: EngineLifecycle,
    label: String,
    id: Option<i32>,
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
            lifecycle: EngineLifecycle::Created,
            label: String::from("unnamed"),
            id: Option::None,
        }
    }

    pub fn with_label(label: &str) -> Self {
        Self {
            window_manager: WindowManager::new(),
            lifecycle: EngineLifecycle::Created,
            label: label.to_string(),
            id: Option::None,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    pub fn lifecycle(&self) -> &EngineLifecycle {
        &self.lifecycle
    }

    pub fn is_running(&self) -> bool {
        self.lifecycle == EngineLifecycle::Running
    }

    pub fn is_paused(&self) -> bool {
        self.lifecycle == EngineLifecycle::Paused
    }

    pub fn is_shutdown(&self) -> bool {
        self.lifecycle == EngineLifecycle::Shutdown
    }

    pub fn pause(&mut self) -> Result<(), EngineError> {
        if self.lifecycle != EngineLifecycle::Running {
            return Err(StateError::InvalidOperation(format!(
                "cannot pause engine in state '{}'",
                self.lifecycle
            ))
            .into());
        }
        self.lifecycle = EngineLifecycle::Paused;
        debug!("Engine '{}' paused", self.label);
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), EngineError> {
        if self.lifecycle != EngineLifecycle::Paused {
            return Err(StateError::InvalidOperation(format!(
                "cannot resume engine in state '{}'",
                self.lifecycle
            ))
            .into());
        }
        self.lifecycle = EngineLifecycle::Running;
        debug!("Engine '{}' resumed", self.label);
        Ok(())
    }

    pub fn shutdown(&mut self) {
        info!("Engine '{}' shutting down", self.label);
        self.window_manager.destroy_all();
        self.lifecycle = EngineLifecycle::Shutdown;
    }

    pub fn create_window(&mut self, name: &str, width: u32, height: u32) -> usize {
        let id = self.window_manager.register_window(
            name,
            width,
            height,
            crate::core::window::WindowGraphics::None,
        );
        debug!("Engine '{}' created window {}", self.label, id);
        id
    }

    pub fn create_window_persistent(&mut self, name: &str, width: u32, height: u32) -> usize {
        let id = self.window_manager.register_window_persistent(
            name,
            width,
            height,
            crate::core::window::WindowGraphics::None,
        );
        debug!("Engine '{}' created persistent window {}", self.label, id);
        id
    }

    pub fn set_window_persist(&mut self, id: usize, persist: bool) -> Result<(), EngineError> {
        self.window_manager.set_persist(id, persist)
    }

    pub fn set_window_persist_by_name(
        &mut self,
        name: &str,
        persist: bool,
    ) -> Result<(), EngineError> {
        self.window_manager.set_persist_by_name(name, persist)
    }

    pub fn destroy_window(&mut self, id: usize) -> Result<(), EngineError> {
        self.window_manager.destroy(id)
    }

    pub fn destroy_window_by_name(&mut self, name: &str) -> Result<(), EngineError> {
        self.window_manager.destroy_by_name(name)
    }

    pub fn destroy_all_windows(&mut self) {
        self.window_manager.destroy_all();
    }

    pub fn list_windows(&self) -> Vec<(usize, &WindowDescriptor)> {
        self.window_manager.list_windows()
    }

    pub fn list_all_windows(&self) -> Vec<(usize, &WindowDescriptor, &WindowState)> {
        self.window_manager.list_all_windows()
    }

    pub fn get_window_id_by_name(&self, name: &str) -> Option<usize> {
        self.window_manager.get_id_by_name(name)
    }

    pub fn get_window_ids_by_name(&self, name: &str) -> Vec<usize> {
        self.window_manager.get_ids_by_name(name)
    }

    pub fn get_window_descriptor_by_name(
        &self,
        name: &str,
    ) -> Result<&WindowDescriptor, EngineError> {
        self.window_manager.get_descriptor_by_name(name)
    }

    pub fn window_descriptor(&self, id: usize) -> Result<&WindowDescriptor, EngineError> {
        self.window_manager.descriptor(id)
    }

    pub fn rename_window(&mut self, id: usize, new_title: &str) -> Result<(), EngineError> {
        self.window_manager.rename_window(id, new_title)
    }

    pub fn resize_window(&mut self, id: usize, width: u32, height: u32) -> Result<(), EngineError> {
        self.window_manager.resize_window(id, width, height)
    }

    pub fn set_primary_window(&mut self, id: usize) -> Result<(), EngineError> {
        self.window_manager.set_primary(id)
    }

    pub fn primary_window_id(&self) -> Option<usize> {
        self.window_manager.primary_id()
    }

    pub fn active_window_count(&self) -> usize {
        self.window_manager.active_count()
    }

    pub fn total_window_count(&self) -> usize {
        self.window_manager.total_count()
    }

    pub fn window_exists(&self, id: usize) -> bool {
        self.window_manager.window_exists(id)
    }

    pub fn run(&mut self) -> Result<(), EngineError> {
        if self.lifecycle == EngineLifecycle::Shutdown {
            return Err(
                StateError::InvalidOperation("cannot run a shut-down engine".to_string()).into(),
            );
        }

        info!("Engine '{}' starting", self.label);
        self.lifecycle = EngineLifecycle::Running;

        self.window_manager.run_all(|event, _window| match event {
            Event::CloseRequested => ControlFlow::Exit,
            Event::KeyDown(Key::Escape) => ControlFlow::Exit,
            _ => ControlFlow::Continue,
        })?;

        self.lifecycle = EngineLifecycle::Shutdown;
        info!("Engine '{}' shut down cleanly", self.label);
        Ok(())
    }

    pub fn run_with<F>(mut self, mut event_handler: F) -> Result<(), EngineError>
    where
        F: FnMut(Event, &windowed::Window) -> ControlFlow,
    {
        if self.lifecycle == EngineLifecycle::Shutdown {
            return Err(
                StateError::InvalidOperation("cannot run a shut-down engine".to_string()).into(),
            );
        }

        info!("Engine '{}' starting with custom handler", self.label);
        self.lifecycle = EngineLifecycle::Running;

        self.window_manager.run_all(&mut event_handler)?;

        self.lifecycle = EngineLifecycle::Shutdown;
        info!("Engine '{}' shut down cleanly", self.label);
        Ok(())
    }

    pub fn run_window<F>(mut self, id: usize, mut event_handler: F) -> Result<(), EngineError>
    where
        F: FnMut(Event, &windowed::Window) -> ControlFlow,
    {
        if self.lifecycle == EngineLifecycle::Shutdown {
            return Err(
                StateError::InvalidOperation("cannot run a shut-down engine".to_string()).into(),
            );
        }

        info!("Engine '{}' starting single window {}", self.label, id);
        self.lifecycle = EngineLifecycle::Running;

        self.window_manager.run(id, &mut event_handler)?;

        self.lifecycle = EngineLifecycle::Shutdown;
        info!("Engine '{}' shut down cleanly", self.label);
        Ok(())
    }
}
