pub trait VecExt<T> {
    /// Returns true if the vector is empty or contains only elements that satisfy the predicate
    fn all_or_empty<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&T) -> bool;
        
    /// Safe way to get the first element as an Option
    fn first_option(&self) -> Option<&T>;
    
    /// Safe way to get the last element as an Option
    fn last_option(&self) -> Option<&T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn all_or_empty<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        self.is_empty() || self.iter().all(predicate)
    }
    
    fn first_option(&self) -> Option<&T> {
        self.first()
    }
    
    fn last_option(&self) -> Option<&T> {
        self.last()
    }
}

/// Extensions for vectors containing Result types
pub trait ResultVecExt<T, E> {
    /// Converts a Vec<Result<T, E>> into a Result<Vec<T>, E>
    /// Collects all values if all are Ok, or returns the first Err
    fn collect_results(self) -> Result<Vec<T>, E>;
}

impl<T, E> ResultVecExt<T, E> for Vec<Result<T, E>> 
where 
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn collect_results(self) -> Result<Vec<T>, E> {
        let mut results = Vec::with_capacity(self.len());
        
        for item in self {
            match item {
                Ok(value) => results.push(value),
                Err(e) => return Err(e),
            }
        }
        
        Ok(results)
    }
}
