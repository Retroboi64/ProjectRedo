use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    Window(windowed::Error),
    State(StateError),
}

#[derive(Debug)]
pub enum StateError {
    NoWindowsRegistered,
    AlreadyRunning,
    WindowIndexOutOfBounds(usize),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Window(e) => write!(f, "window error: {:?}", e),
            EngineError::State(e) => write!(f, "state error: {}", e),
        }
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateError::NoWindowsRegistered => write!(f, "no windows have been registered"),
            StateError::AlreadyRunning => write!(f, "engine is already running"),
            StateError::WindowIndexOutOfBounds(i) => {
                write!(f, "window index {} is out of bounds", i)
            }
        }
    }
}

impl std::error::Error for EngineError {}

/// Lets `?` convert windowed errors automatically.
impl From<windowed::Error> for EngineError {
    fn from(e: windowed::Error) -> Self {
        EngineError::Window(e)
    }
}

impl From<StateError> for EngineError {
    fn from(e: StateError) -> Self {
        EngineError::State(e)
    }
}
