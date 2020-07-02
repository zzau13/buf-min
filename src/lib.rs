// Adapted from [`bytes`](https://github.com/tokio-rs/bytes)

use std::vec::Vec;

#[cfg(feature = "bytes-buf")]
use bytes::{Bytes, BytesMut};

/// Minimal Buffer trait
pub trait Buffer {
    /// Into immutable type
    type Freeze;

    /// Returns new `Buffer` with capacity
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    /// Appends given bytes to this `Buffer`.
    ///
    /// # Panics
    /// Can panic if current length plus `src` length overflows usize
    fn extend_from_slice(&mut self, src: &[u8]);

    /// Reserves capacity for at least `additional` more bytes to be inserted
    /// into the given `Buffer`.
    ///
    /// # Panics
    /// Can panic if current capacity plus `additional` overflows usize
    fn reserve(&mut self, additional: usize);

    /// Converts `self` into a Freeze type
    fn freeze(self) -> Self::Freeze;

    /// Advance the internal cursor of the `Buffer`
    ///
    /// # Safety
    /// Can't advance more than capacity of the `Buffer`
    ///
    /// # Panics
    /// Can panic if length plus `cnt` is bigger than capacity
    unsafe fn advance(&mut self, cnt: usize);

    /// Return unsafe ptr to current `Buffer` position
    ///
    /// # Safety
    /// If buffer is full, can return invalid pointer
    unsafe fn buf_ptr(&mut self) -> *mut u8;
}

impl Buffer for Vec<u8> {
    type Freeze = Vec<u8>;

    #[inline]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn extend_from_slice(&mut self, src: &[u8]) {
        self.reserve(src.len());
        unsafe {
            debug_assert!(self.capacity() - self.len() >= src.len());
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
            self.advance(src.len())
        }
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        let len = self.len();
        let rem = self.capacity() - len;

        if additional <= rem {
            return;
        }
        self.reserve(additional);
    }

    #[inline]
    fn freeze(self) -> Self::Freeze {
        self
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        let new_len = self.len() + cnt;
        debug_assert!(
            new_len <= self.capacity(),
            "new_len = {}; capacity = {}",
            new_len,
            self.capacity()
        );
        self.set_len(new_len);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

#[cfg(feature = "bytes-buf")]
impl Buffer for BytesMut {
    type Freeze = Bytes;

    #[inline]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        BytesMut::with_capacity(capacity)
    }

    #[inline]
    fn extend_from_slice(&mut self, src: &[u8]) {
        self.reserve(src.len());
        unsafe {
            debug_assert!(self.capacity() - self.len() >= src.len());
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
            self.advance(src.len())
        }
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    #[inline]
    fn freeze(self) -> Self::Freeze {
        self.freeze()
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        let new_len = self.len() + cnt;
        debug_assert!(
            new_len <= self.capacity(),
            "new_len = {}; capacity = {}",
            new_len,
            self.capacity()
        );
        self.set_len(new_len);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let e = b"Hello world!";
        let mut buf: Vec<u8> = Buffer::with_capacity(0);
        Buffer::extend_from_slice(&mut buf, e);
        let buf: &[u8] = &Buffer::freeze(buf);
        assert_eq!(e, buf)
    }
}
