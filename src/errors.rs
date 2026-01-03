use std::error::Error;

/// Shorter alias for `Box<dyn Error>`
pub type AnyErr = Box<dyn Error>;
