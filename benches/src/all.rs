use std::mem::MaybeUninit;
use std::slice;

use criterion::{criterion_group, criterion_main, Criterion};

use buf_min::{Buffer, t3::BytesMut};

criterion_group!(benches, functions);
criterion_main!(benches);

fn functions(c: &mut Criterion) {
    c.bench_function("Raw \"Super\" Static", raw_sstatic);
    c.bench_function("Raw Static", raw_static);
    c.bench_function("Raw Dyn", raw_dyn);
    c.bench_function("Buffer Bytes", buffer_bytes);
}

// Raw
const HELLO: &[u8] = b"Hello world!";
fn raw_static(b: &mut criterion::Bencher) {
    unsafe {
        const LEN: usize = HELLO.len();

        b.iter(|| {
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
        });
    }
}

fn raw_sstatic(b: &mut criterion::Bencher) {
    unsafe {
        b.iter(|| {
            const LEN: usize = HELLO.len();

            let mut buf: [MaybeUninit<u8>; LEN] = [MaybeUninit::uninit(); LEN];
            macro_rules! buf_ptr {
                () => {
                    &mut buf as *mut _ as *mut u8
                };
            }

            macro_rules! write_b {
                ($b:expr) => {
                    if LEN < $b.len() {
                        panic!("buffer overflow");
                    } else {
                        std::ptr::copy_nonoverlapping(
                            ($b as *const [u8] as *const u8),
                            buf_ptr!(),
                            $b.len(),
                        );
                    }
                };
            }

            write_b!(HELLO);
            std::mem::transmute::<_, [u8; HELLO.len()]>(buf)
        });
    }
}

fn raw_dyn(b: &mut criterion::Bencher) {
    unsafe {
        const LEN: usize = HELLO.len();

        b.iter(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(LEN);
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
