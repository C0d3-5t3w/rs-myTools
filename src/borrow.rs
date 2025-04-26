use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};

/// A trait for safely taking ownership of a value temporarily and then putting it back
pub trait TakeReplace<T>: Sized {
    /// Take ownership of the value, leaving a default in its place
    fn take(&mut self) -> T;
    
    /// Replace the current value with another
    fn replace(&mut self, value: T) -> T;
    
    /// Update value in-place using a function
    fn update<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
        T: Default,
    {
        let old_val = self.take();
        self.replace(f(old_val));
    }
}

impl<T: Default> TakeReplace<T> for T {
    fn take(&mut self) -> T {
        std::mem::take(self)
    }
    
    fn replace(&mut self, value: T) -> T {
        std::mem::replace(self, value)
    }
}

/// A wrapper for safely handling self-referential structures
/// by using interior mutability
pub struct SelfRef<T> {
    inner: RefCell<T>,
}

impl<T> SelfRef<T> {
    /// Create a new self-referential wrapper
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    
    /// Borrow the value immutably
    pub fn borrow(&self) -> impl Deref<Target = T> + '_ {
        self.inner.borrow()
    }
    
    /// Borrow the value mutably
    pub fn borrow_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.inner.borrow_mut()
    }
    
    /// Apply a function to the value that returns a result
    pub fn with<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        f(&self.inner.borrow())
    }
    
    /// Apply a function that mutates the value
    pub fn with_mut<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        f(&mut self.inner.borrow_mut())
    }
}

impl<T: Default> Default for SelfRef<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// A trait for temporarily borrowing multiple fields without 
/// running into borrow checker conflicts
pub trait SplitBorrow {
    /// Split a struct into multiple borrowable parts
    /// to avoid the borrow checker's constraints on 
    /// borrowing multiple fields at once
    fn split<'a>(&'a mut self) -> Self::SplitParts<'a> where Self: 'a;
    
    /// The type returned when the struct is split
    type SplitParts<'a> where Self: 'a;
}

// Implementation for a 2-field struct
impl<T: 'static, U: 'static> SplitBorrow for (T, U) {
    type SplitParts<'a> = (&'a mut T, &'a mut U) where T: 'a, U: 'a;
    
    fn split<'a>(&'a mut self) -> Self::SplitParts<'a> where Self: 'a {
        let (first, second) = self;
        (first, second)
    }
}

/// Implementation for a 3-field struct
impl<T: 'static, U: 'static, V: 'static> SplitBorrow for (T, U, V) {
    type SplitParts<'a> = (&'a mut T, &'a mut U, &'a mut V) where T: 'a, U: 'a, V: 'a;
    
    fn split<'a>(&'a mut self) -> Self::SplitParts<'a> where Self: 'a {
        let (first, second, third) = self;
        (first, second, third)
    }
}

/// Helper struct to split fields of arbitrary structs
pub struct FieldSplit<'a, T> {
    parent: &'a mut T,
}

impl<'a, T> FieldSplit<'a, T> {
    /// Create a new field splitter from a mutable reference
    pub fn new(parent: &'a mut T) -> Self {
        Self { parent }
    }
    
    /// Get a reference to a field using a projection function
    pub fn field<F, U>(&mut self, proj: F) -> &mut U 
    where 
        F: FnOnce(&mut T) -> &mut U
    {
        proj(self.parent)
    }
}

/// A clone-on-write wrapper for types that don't implement Clone
/// Useful when you need to make copies selectively during runtime
pub struct CowCell<T> {
    inner: RefCell<T>,
}

impl<T> CowCell<T> {
    /// Create a new clone-on-write cell
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    
    /// Get immutable access to the value
    pub fn get(&self) -> impl Deref<Target = T> + '_ {
        self.inner.borrow()
    }
    
    /// Get mutable access to the value
    pub fn get_mut(&self) -> impl DerefMut<Target = T> + '_ {
        self.inner.borrow_mut()
    }
    
    /// Apply an operation that may need to clone the value
    pub fn with_cow<R, F, C>(&self, clone_fn: C, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
        C: FnOnce(&T) -> T,
    {
        let cloned = clone_fn(&self.inner.borrow());
        let mut borrowed = self.inner.borrow_mut();
        *borrowed = cloned;
        f(&mut borrowed)
    }
}

/// A wrapper for values that can be mutated behind a shared reference
pub struct MutShared<T>(Cell<T>);

impl<T: Copy> MutShared<T> {
    /// Create a new wrapper allowing mutation through shared references
    pub fn new(value: T) -> Self {
        Self(Cell::new(value))
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.0.get()
    }
    
    /// Set a new value
    pub fn set(&self, value: T) {
        self.0.set(value)
    }
    
    /// Update the value using a function
    pub fn update<F: FnOnce(T) -> T>(&self, f: F) {
        let old = self.get();
        self.set(f(old));
    }
}
