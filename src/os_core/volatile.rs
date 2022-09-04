//! This module implements volatile read and writes.
//!
//! A volatile value indicates that the given value may change between different accesses even if
//! it does not appear to be modified. By marking a value of type T as Volatile<T> we tell the
//! compiler that it should **not** optimize read and writes reusing a stale value or ommiting
//! writes.
//!
//! For more information:
//! https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
//! https://doc.rust-lang.org/core/ptr/fn.read_volatile.html
use core::ops::{Deref, DerefMut};
use core::ptr::{read_volatile, write_volatile};

/// Allow to perform volatile read and writes of the wrapped value.
/// This field will always be treated as a raw pointer to data of type R. For that reason, the
/// implementations of `new` and `new_mutable` methods always receive a reference.
#[repr(transparent)]
pub struct Volatile<R>(R);

impl<'a, R> Volatile<&'a mut R> {
    pub fn new_mutable(reference: &'a mut R) -> Self {
        Self(reference)
    }
}

impl<'a, R> Volatile<&'a R> {
    pub const fn new(reference: &'a R) -> Self {
        Self(reference)
    }
}

impl<R, T> Volatile<R>
where
    R: DerefMut<Target = T>,
{
    /// Performs a volatile write of the given value
    pub fn write(&mut self, value: T) {
        // Define a raw pointer to the data we are going to write.
        // self.0: R
        // *self.0: T
        // &mut *self.0: &mut T
        let destiny = &mut *self.0 as *mut T;
        // Write it :)
        unsafe { write_volatile(destiny, value) };
    }
}

impl<R, T> Volatile<R>
where
    R: Deref<Target = T>,
{
    /// Performs a volatile read of the given value
    pub fn read(&self) -> T {
        // Define a raw pointer to the data we are going to read.
        // self.0: R
        // *self.0: T
        // &*self.0: &T
        let source = &*self.0 as *const T;
        // Return it :)
        unsafe { read_volatile(source) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn read_ok() {
        let val = 255;
        let volatile = Volatile::new(&val);

        assert_eq!(255, volatile.read());
    }

    #[test_case]
    fn write_ok() {
        let mut val = 255;
        let new_val = 42;
        let mut volatile = Volatile::new_mutable(&mut val);
        volatile.write(new_val);

        assert_eq!(new_val, volatile.read());
        assert_eq!(new_val, val);
    }
}
