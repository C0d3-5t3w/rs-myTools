pub trait ResultExt<T, E> {
    /// Ignore the error case and convert to Option
    fn ignore_err(self) -> Option<T>;
    
    /// Apply function to error case
    fn map_err_with<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> E;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn ignore_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    
    fn map_err_with<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&E) -> E,
    {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(f(&e)),
        }
    }
}
