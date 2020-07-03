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

            let mut buf: Vec<u8>  = Vec::with_capacity(LEN);
            let mut curr = 0;
            macro_rules! buf_ptr {
                () => {
                    buf.as_mut_ptr()
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
            buf.set_len(curr);
            buf
    }
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
    let _ = buffer_bytes();
    let _ = raw_static();
}
