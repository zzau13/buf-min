use std::mem::MaybeUninit;
use std::slice;

use buf_min::Buffer;
use bytes::BytesMut;

// Raw
const HELLO: &[u8] = b"Hello world!";

#[inline(never)]
fn raw_static() -> Vec<u8> {
    unsafe {
        const LEN: usize = HELLO.len();

        let mut buf: [MaybeUninit<u8>; LEN] = [MaybeUninit::uninit(); LEN];
        let mut curr = 0;
        macro_rules! buf_ptr {
            () => {
                &mut buf as *mut _ as *mut u8
            };
        }

        macro_rules! write_b {
            ($b:expr) => {
                if LEN < curr + $b.len() {
                    panic!("buffer overflow");
                } else {
                    std::ptr::copy_nonoverlapping(
                        ($b as *const [u8] as *const u8),
                        buf_ptr!().add(curr),
                        $b.len(),
                    );
                    curr += $b.len();
                }
            };
        }

        write_b!(HELLO);
        slice::from_raw_parts(&buf as *const _ as *const u8, curr).to_vec()
    }
}

#[inline(never)]
fn raw_dyn() -> Vec<u8> {
    unsafe {
        const LEN: usize = HELLO.len();

        let mut buf: Vec<u8> = Vec::with_capacity(LEN);
        #[allow(unused_mut)]
        let mut capacity = LEN;
        let mut curr = 0;
        macro_rules! buf_ptr {
            () => {
                buf.as_mut_ptr()
            };
        }

        macro_rules! write_b {
            ($b:expr) => {
                if capacity < curr + $b.len() {
                    panic!("buffer overflow");
                } else {
                    std::ptr::copy_nonoverlapping(
                        ($b as *const [u8] as *const u8),
                        buf_ptr!().add(curr),
                        $b.len(),
                    );
                    curr += $b.len();
                }
            };
        }

        write_b!(HELLO);
        buf.set_len(curr);
        buf
    }
}

struct IBuff {
    pub ptr: Vec<u8>,
    pub len: usize,
    cap: usize,
}

impl IBuff {
    #[inline]
    fn new(c: usize) -> Self {
        IBuff {
            ptr: Vec::with_capacity(c),
            len: 0,
            cap: c,
        }
    }

    #[inline(always)]
    fn write(&mut self, s: &[u8]) {
        if self.cap < self.len + s.len(){
            panic!("buffer overflow");
        } else {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    s.as_ptr(),
                    self.ptr.as_mut_ptr().add(self.len),
                    s.len(),
                );
            }
            self.len += s.len();
        }
    }

    #[inline]
    fn freeze(mut self) -> Vec<u8> {
        unsafe { self.ptr.set_len(self.len) }
        self.ptr
    }
}

#[inline(never)]
fn ibuffer() -> Vec<u8> {
    const LEN: usize = HELLO.len();

    let mut buf: IBuff = IBuff::new(LEN);
    buf.write(HELLO);
    buf.freeze()
}

// Buffer
#[inline(never)]
fn buffer_bytes() -> BytesMut {
    const LEN: usize = HELLO.len();

    let mut buf: BytesMut = Buffer::with_capacity(LEN);
    Buffer::extend_from_slice(&mut buf, HELLO);
    buf
}

fn main() {
    let _ = raw_static();
    let _ = raw_dyn();
    let _ = ibuffer();
    let _ = buffer_bytes();
}
