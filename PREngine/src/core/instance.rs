#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use probably_fine_log::{debug, info, warn};

use crate::core::{
    engine::{Engine, EngineLifecycle},
    error::{EngineError, StateError},
    window::WindowDescriptor,
};

pub type EngineId = usize;

struct EngineEntry {
    engine: Engine,
    name: String,
    created_at: Instant,
}

pub struct EngineRegistry {
    entries: HashMap<EngineId, EngineEntry>,
    next_id: EngineId,
}

static REGISTRY: OnceLock<Mutex<EngineRegistry>> = OnceLock::new();

impl EngineRegistry {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn global() -> &'static Mutex<EngineRegistry> {
        REGISTRY.get_or_init(|| Mutex::new(EngineRegistry::new()))
    }

    pub fn spawn(name: &str) -> EngineId {
        let mut reg = Self::global().lock().unwrap();
        let id = reg.next_id;
        reg.next_id += 1;
        reg.entries.insert(
            id,
            EngineEntry {
                engine: Engine::with_label(name),
                name: name.to_string(),
                created_at: Instant::now(),
            },
        );
        info!("Engine '{}' spawned with id {}", name, id);
        id
    }

    pub fn destroy(id: EngineId) -> Result<(), EngineError> {
        let mut reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .get_mut(&id)
            .ok_or(StateError::EngineNotFound(id))?;
        if entry.engine.is_running() {
            warn!(
                "Destroying engine '{}' (id {}) while running",
                entry.name, id
            );
        }
        entry.engine.shutdown();
        let name = entry.name.clone();
        reg.entries.remove(&id);
        info!("Engine '{}' (id {}) destroyed", name, id);
        Ok(())
    }

    pub fn destroy_by_name(name: &str) -> Result<(), EngineError> {
        let id = {
            let reg = Self::global().lock().unwrap();
            reg.entries
                .iter()
                .find(|(_, e)| e.name == name)
                .map(|(id, _)| *id)
                .ok_or_else(|| StateError::EngineNameNotFound(name.to_string()))?
        };
        Self::destroy(id)
    }

    pub fn destroy_all() {
        let mut reg = Self::global().lock().unwrap();
        let count = reg.entries.len();
        for entry in reg.entries.values_mut() {
            entry.engine.shutdown();
        }
        reg.entries.clear();
        info!("Destroyed all {} engines", count);
    }

    pub fn list() -> Vec<(EngineId, String, EngineLifecycle)> {
        let reg = Self::global().lock().unwrap();
        let mut list: Vec<_> = reg
            .entries
            .iter()
            .map(|(id, e)| (*id, e.name.clone(), e.engine.lifecycle().clone()))
            .collect();
        list.sort_by_key(|(id, _, _)| *id);
        list
    }

    pub fn get_id_by_name(name: &str) -> Option<EngineId> {
        let reg = Self::global().lock().unwrap();
        reg.entries
            .iter()
            .find(|(_, e)| e.name == name)
            .map(|(id, _)| *id)
    }

    pub fn get_ids_by_name(name: &str) -> Vec<EngineId> {
        let reg = Self::global().lock().unwrap();
        let mut ids: Vec<_> = reg
            .entries
            .iter()
            .filter(|(_, e)| e.name == name)
            .map(|(id, _)| *id)
            .collect();
        ids.sort();
        ids
    }

    pub fn exists(id: EngineId) -> bool {
        Self::global().lock().unwrap().entries.contains_key(&id)
    }

    pub fn count() -> usize {
        Self::global().lock().unwrap().entries.len()
    }

    pub fn uptime(id: EngineId) -> Result<Duration, EngineError> {
        let reg = Self::global().lock().unwrap();
        reg.entries
            .get(&id)
            .map(|e| e.created_at.elapsed())
            .ok_or(StateError::EngineNotFound(id).into())
    }

    pub fn rename(id: EngineId, new_name: &str) -> Result<(), EngineError> {
        let mut reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .get_mut(&id)
            .ok_or(StateError::EngineNotFound(id))?;
        debug!(
            "Renaming engine {} from '{}' to '{}'",
            id, entry.name, new_name
        );
        entry.name = new_name.to_string();
        entry.engine.set_label(new_name);
        Ok(())
    }

    pub fn lifecycle(id: EngineId) -> Result<EngineLifecycle, EngineError> {
        let reg = Self::global().lock().unwrap();
        reg.entries
            .get(&id)
            .map(|e| e.engine.lifecycle().clone())
            .ok_or(StateError::EngineNotFound(id).into())
    }

    pub fn with<F, R>(id: EngineId, f: F) -> Result<R, EngineError>
    where
        F: FnOnce(&mut Engine) -> R,
    {
        let mut reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .get_mut(&id)
            .ok_or(StateError::EngineNotFound(id))?;
        Ok(f(&mut entry.engine))
    }

    pub fn with_result<F, R>(id: EngineId, f: F) -> Result<R, EngineError>
    where
        F: FnOnce(&mut Engine) -> Result<R, EngineError>,
    {
        let mut reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .get_mut(&id)
            .ok_or(StateError::EngineNotFound(id))?;
        f(&mut entry.engine)
    }

    pub fn create_window(
        id: EngineId,
        name: &str,
        width: u32,
        height: u32,
    ) -> Result<usize, EngineError> {
        Self::with_result(id, |engine| Ok(engine.create_window(name, width, height)))
    }

    pub fn destroy_window(id: EngineId, window_id: usize) -> Result<(), EngineError> {
        Self::with_result(id, |engine| engine.destroy_window(window_id))
    }

    pub fn list_windows(id: EngineId) -> Result<Vec<WindowDescriptor>, EngineError> {
        let reg = Self::global().lock().unwrap();
        let entry = reg.entries.get(&id).ok_or(StateError::EngineNotFound(id))?;
        Ok(entry
            .engine
            .list_windows()
            .into_iter()
            .map(|(_, d)| d.clone())
            .collect())
    }

    pub fn get_window_id_by_name(
        engine_id: EngineId,
        window_name: &str,
    ) -> Result<Option<usize>, EngineError> {
        let reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .get(&engine_id)
            .ok_or(StateError::EngineNotFound(engine_id))?;
        Ok(entry.engine.get_window_id_by_name(window_name))
    }

    pub fn take(id: EngineId) -> Result<Engine, EngineError> {
        let mut reg = Self::global().lock().unwrap();
        let entry = reg
            .entries
            .remove(&id)
            .ok_or(StateError::EngineNotFound(id))?;
        info!("Engine '{}' (id {}) taken from registry", entry.name, id);
        Ok(entry.engine)
    }

    pub fn insert(name: &str, engine: Engine) -> EngineId {
        let mut reg = Self::global().lock().unwrap();
        let id = reg.next_id;
        reg.next_id += 1;
        reg.entries.insert(
            id,
            EngineEntry {
                engine,
                name: name.to_string(),
                created_at: Instant::now(),
            },
        );
        info!("Engine '{}' inserted into registry with id {}", name, id);
        id
    }
}
