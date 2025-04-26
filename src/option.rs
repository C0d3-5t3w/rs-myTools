pub trait OptionExt<T> {
    /// Apply a function to the contained value if the option is `Some`, otherwise return `default`
    fn map_or_default<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U;
        
    /// Converts from Option<T> to Option<U> by applying a function to a contained value
    /// or returns None if the Option is None
    fn try_map<U, E, F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn map_or_default<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Some(value) => f(value),
            None => default,
        }
    }
    
    fn try_map<U, E, F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Some(value) => f(value).map(Some),
            None => Ok(None),
        }
    }
}
