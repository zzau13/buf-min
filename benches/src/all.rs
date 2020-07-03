use std::mem::MaybeUninit;
use std::slice;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use buf_min::Buffer;
use bytes::BytesMut;

criterion_group!(benches, functions);
criterion_main!(benches);

fn functions(c: &mut Criterion) {
    // 3 bytes
    c.bench_function("Raw Static", raw_static);
    c.bench_function("Buffer Bytes", buffer_bytes);
}

// Raw
const HELLO: &[u8] = b"Hello world!";
fn raw_static(b: &mut criterion::Bencher) {
    unsafe {
        const LEN: usize = HELLO.len();

        b.iter(|| {
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
        });
    }
}

// Buffer
fn buffer_bytes(b: &mut criterion::Bencher) {
    const LEN: usize = HELLO.len();

    b.iter(|| {
        let mut buf: BytesMut = Buffer::with_capacity(LEN);
        Buffer::extend_from_slice(&mut buf, HELLO);
    });
}
