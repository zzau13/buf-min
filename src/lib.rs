// Adapted from [`bytes`](https://github.com/tokio-rs/bytes)

/// Minimal Buffer trait with utf-8 safety
pub trait Buffer {
    /// Into immutable type
    type Freeze;

    /// Returns new `Buffer` with capacity
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    /// Returns true if the `Buffer` has a length of 0.
    fn is_empty(&self) -> bool;

    /// Appends given str to this `Buffer`.
    ///
    /// # Panics
    /// Can panic if current length plus `src` length overflows usize
    #[inline]
    fn extend(&mut self, src: &str) {
        // SAFETY: utf-8 checked
        unsafe {
            self.extend_from_slice(src.as_bytes());
        }
    }

    fn len(&self) -> usize;

    /// Appends given bytes to this `Buffer`.
    ///
    /// # Safety
    /// Broke utf-8 safety
    ///
    /// # Panics
    /// Can panic if current length plus `src` length overflows usize
    unsafe fn extend_from_slice(&mut self, src: &[u8]);

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
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn extend_from_slice(&mut self, src: &[u8]) {
        Buffer::reserve(self, src.len());
        debug_assert!(self.capacity() - self.len() >= src.len());
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
        Buffer::advance(self, src.len())
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        debug_assert!(self.len() <= self.capacity());
        if self.capacity().wrapping_sub(self.len()) < additional {
            self.reserve(additional);
        }
    }

    #[inline]
    fn freeze(mut self) -> Self::Freeze {
        self.shrink_to_fit();
        self
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        self.set_len(self.len() + cnt);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

impl Buffer for String {
    type Freeze = String;

    #[inline]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        String::with_capacity(capacity)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    unsafe fn extend_from_slice(&mut self, src: &[u8]) {
        Buffer::reserve(self, src.len());
        debug_assert!(self.capacity() - self.len() >= src.len());
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
        Buffer::advance(self, src.len())
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        debug_assert!(self.len() <= self.capacity());
        if self.capacity().wrapping_sub(self.len()) < additional {
            self.reserve(additional);
        }
    }

    #[inline]
    fn freeze(mut self) -> Self::Freeze {
        self.shrink_to_fit();
        self
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        let len = self.len() + cnt;
        self.as_mut_vec().set_len(len);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

#[cfg(any(
    feature = "bytes-buf-tokio2",
    feature = "bytes-buf-tokio3",
    feature = "ntex-bytes"
))]
macro_rules! implement {
    ($base:path) => {
        pub use $base::{Bytes, BytesMut};

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
            fn is_empty(&self) -> bool {
                self.is_empty()
            }

            #[inline]
            fn len(&self) -> usize {
                self.len()
            }

            #[inline]
            unsafe fn extend_from_slice(&mut self, src: &[u8]) {
                Buffer::reserve(self, src.len());
                debug_assert!(self.capacity() - self.len() >= src.len());
                std::ptr::copy_nonoverlapping(src.as_ptr(), Buffer::buf_ptr(self), src.len());
                Buffer::advance(self, src.len());
            }

            #[inline(always)]
            fn reserve(&mut self, additional: usize) {
                self.reserve(additional);
            }

            #[inline(always)]
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
        mod test_bytes {
            use super::*;

            #[test]
            fn test() {
                let e = "Hello world!";
                let mut buf: BytesMut = Buffer::with_capacity(0);
                Buffer::extend(&mut buf, e);
                assert_eq!(e.as_bytes(), &Buffer::freeze(buf)[..]);

                let mut buf: BytesMut = Buffer::with_capacity(124);
                Buffer::extend(&mut buf, e);
                assert_eq!(e.as_bytes(), &Buffer::freeze(buf)[..]);
            }
        }
    };
}

#[cfg(feature = "bytes-buf-tokio2")]
/// tokio/bytes@0.5 implementation and reexport
pub mod t2 {
    use super::*;
    implement!(bytes_tokio2);
}

#[cfg(feature = "bytes-buf-tokio3")]
/// tokio/bytes@1.0 implementation and reexport
pub mod t3 {
    use super::*;
    implement!(bytes_tokio3);
}

#[cfg(feature = "ntex-bytes")]
pub mod ntex {
    use super::*;
    implement!(ntex_bytes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let e = "Hello world!";
        let mut buf: Vec<u8> = Buffer::with_capacity(0);
        Buffer::extend(&mut buf, e);
        assert_eq!(e.as_bytes(), &Buffer::freeze(buf)[..]);

        let mut buf: Vec<u8> = Buffer::with_capacity(124);
        Buffer::extend(&mut buf, e);
        assert_eq!(e.as_bytes(), &Buffer::freeze(buf)[..]);

        let mut buf: Vec<u8> = Buffer::with_capacity(14);
        Buffer::extend(&mut buf, e);
    }
}
