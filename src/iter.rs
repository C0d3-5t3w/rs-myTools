pub trait IterExt: Iterator {
    /// Returns the first `n` elements as a Vec
    fn take_vec(self, n: usize) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.take(n).collect()
    }
    
    /// Returns every nth element of the iterator
    fn every_nth(self, n: usize) -> EveryNth<Self>
    where
        Self: Sized,
    {
        assert!(n > 0, "n must be greater than 0");
        EveryNth { iter: self, n, index: 0 }
    }
}

impl<T: Iterator> IterExt for T {}

/// Iterator adapter that yields every nth element
pub struct EveryNth<I> {
    iter: I,
    n: usize,
    index: usize,
}

impl<I: Iterator> Iterator for EveryNth<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next()?;
            let index = self.index;
            self.index = self.index + 1;
            
            if index % self.n == 0 {
                return Some(item);
            }
        }
    }
}
