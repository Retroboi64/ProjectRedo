#[derive(Debug)]
pub enum CallError {
    /// Symbol was not found in the library.
    Load(libloading::Error),
}

impl From<libloading::Error> for CallError {
    fn from(e: libloading::Error) -> Self {
        CallError::Load(e)
    }
}
