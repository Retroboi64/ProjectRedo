use std::collections::HashSet;
use windowed::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    JustPressed,
    Held,
    JustReleased,
}

pub struct Input {
    down: HashSet<Key>,
    just_pressed: HashSet<Key>,
    just_released: HashSet<Key>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            down: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub(crate) fn process_key_down(&mut self, key: Key) {
        if self.down.insert(key) {
            self.just_pressed.insert(key);
        }
    }

    pub(crate) fn process_key_up(&mut self, key: Key) {
        self.down.remove(&key);
        self.just_released.insert(key);
    }

    pub(crate) fn flush(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn held(&self, key: Key) -> bool {
        self.down.contains(&key)
    }

    pub fn just_released(&self, key: Key) -> bool {
        self.just_released.contains(&key)
    }

    pub fn state(&self, key: Key) -> Option<KeyState> {
        if self.just_pressed.contains(&key) {
            Some(KeyState::JustPressed)
        } else if self.just_released.contains(&key) {
            Some(KeyState::JustReleased)
        } else if self.down.contains(&key) {
            Some(KeyState::Held)
        } else {
            None
        }
    }

    pub fn active_keys(&self) -> impl Iterator<Item = &Key> {
        self.down.iter()
    }

    pub fn is_idle(&self) -> bool {
        self.down.is_empty()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
