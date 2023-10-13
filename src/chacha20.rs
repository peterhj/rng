use crate::{Generator, Buffer32};

use byteorder::{ReadBytesExt, LittleEndian as LE};

use std::io::{Read, Seek, SeekFrom, Cursor, Error as IoError};

#[inline]
pub fn chacha20_quarter_round(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
  *a = (*a).wrapping_add(*b); *d = (*d ^ *a).rotate_left(16);
  *c = (*c).wrapping_add(*d); *b = (*b ^ *c).rotate_left(12);
  *a = (*a).wrapping_add(*b); *d = (*d ^ *a).rotate_left(8);
  *c = (*c).wrapping_add(*d); *b = (*b ^ *c).rotate_left(7);
}

pub fn chacha20_next(state: &mut [u32; 16], out: &mut [u32; 16]) {
  let mut x = *state;
  let mut ctr = (x[12] as u64) | ((x[13] as u64) << 32);
  {
    let [mut x0, mut x1, mut x2, mut x3,
         mut x4, mut x5, mut x6, mut x7,
         mut x8, mut x9, mut x10, mut x11,
         mut x12, mut x13, mut x14, mut x15] = x;
    for _ in 0 .. 10 {
      chacha20_quarter_round(&mut x0, &mut x4, &mut x8, &mut x12);
      chacha20_quarter_round(&mut x1, &mut x5, &mut x9, &mut x13);
      chacha20_quarter_round(&mut x2, &mut x6, &mut x10, &mut x14);
      chacha20_quarter_round(&mut x3, &mut x7, &mut x11, &mut x15);
      chacha20_quarter_round(&mut x0, &mut x5, &mut x10, &mut x15);
      chacha20_quarter_round(&mut x1, &mut x6, &mut x11, &mut x12);
      chacha20_quarter_round(&mut x2, &mut x7, &mut x8, &mut x13);
      chacha20_quarter_round(&mut x3, &mut x4, &mut x9, &mut x14);
    }
    x = [x0, x1, x2, x3,
         x4, x5, x6, x7,
         x8, x9, x10, x11,
         x12, x13, x14, x15];
  }
  for k in 0 .. 16 {
    x[k] = x[k].wrapping_add(state[k]);
  }
  ctr += 1;
  state[12] = ctr as u32;
  state[13] = (ctr >> 32) as u32;
  *out = x;
}

pub struct ChaCha20Generator {
  state: [u32; 16],
}

impl From<[u32; 16]> for ChaCha20Generator {
  fn from(state: [u32; 16]) -> ChaCha20Generator {
    ChaCha20Generator{state}
  }
}

impl ChaCha20Generator {
  pub fn from_parts<C: AsRef<[u8]>, K: AsRef<[u8]>>(constant_buf: C, key_buf: K, nonce: u64, ctr: u64) -> ChaCha20Generator {
    let mut state = [0; 16];
    let mut constant = Cursor::new(constant_buf.as_ref());
    for k in 0 .. 4 {
      state[k] = constant.read_u32::<LE>().unwrap();
    }
    drop(constant);
    let mut key = Cursor::new(key_buf.as_ref());
    for k in 4 .. 12 {
      state[k] = key.read_u32::<LE>().unwrap();
    }
    drop(key);
    state[12] = ctr as u32;
    state[13] = (ctr >> 32) as u32;
    state[14] = nonce as u32;
    state[15] = (nonce >> 32) as u32;
    ChaCha20Generator{state}
  }

  pub fn new_default<R: Read>(mut key_seed: R, nonce: u64, ctr: u64) -> ChaCha20Generator {
    let mut key_buf = [0; 32];
    key_seed.read_exact(&mut key_buf).unwrap();
    ChaCha20Generator::from_parts(b"extend 32-byte k", key_buf, nonce, ctr)
  }
}

impl Generator<[u32; 16]> for ChaCha20Generator {
  #[inline]
  fn next_gen(&mut self, out: &mut [u32; 16]) {
    chacha20_next(&mut self.state, out);
  }
}

impl Seek for ChaCha20Generator {
  fn seek(&mut self, pos: SeekFrom) -> Result<u64, IoError> {
    match pos {
      SeekFrom::Start(p) => {
        assert_eq!(p % 64, 0);
        let ctr = p / 64;
        self.state[12] = ctr as u32;
        self.state[13] = (ctr >> 32) as u32;
        Ok(p)
      }
      _ => unimplemented!()
    }
  }

  fn stream_position(&mut self) -> Result<u64, IoError> {
    let ctr = (self.state[12] as u64) | ((self.state[13] as u64) << 32);
    Ok(ctr * 64)
  }
}

pub type ChaCha20Stream = Buffer32<ChaCha20Generator, [u32; 16]>;

#[cfg(test)]
mod tests {
use super::{ChaCha20Generator, chacha20_quarter_round};
use crate::{Generator};

#[test]
fn test_chacha20_ietf_test_vector_2_1_1() {
  let mut a: u32 = 0x11111111;
  let mut b: u32 = 0x01020304;
  let mut c: u32 = 0x9b8d6f43;
  let mut d: u32 = 0x01234567;
  chacha20_quarter_round(&mut a, &mut b, &mut c, &mut d);
  assert_eq!(a, 0xea2a92f4);
  assert_eq!(b, 0xcb1cf8ce);
  assert_eq!(c, 0x4581472e);
  assert_eq!(d, 0x5881c4bb);
}

#[test]
fn test_chacha20_ietf_test_vector_2_3_1() {
  let constant_buf: [u8; 16] = [
      0x65, 0x78, 0x70, 0x61,
      0x6e, 0x64, 0x20, 0x33,
      0x32, 0x2d, 0x62, 0x79,
      0x74, 0x65, 0x20, 0x6b,
  ];
  let mut key_buf: [u8; 32] = [0; 32];
  for k in 0 .. 32 {
    key_buf[k as usize] = k;
  }
  let ctr: u64 = 0x900_0000_0000_0001;
  let nonce: u64 = 0x4a00_0000;
  let mut gen = ChaCha20Generator::from_parts(constant_buf, key_buf, nonce, ctr);
  let mut out = [0; 16];
  gen.next_gen(&mut out);
  assert_eq!(out[0], 0xe4e7f110);
  assert_eq!(out[1], 0x15593bd1);
  assert_eq!(out[2], 0x1fdd0f50);
  assert_eq!(out[3], 0xc47120a3);
  assert_eq!(out[4], 0xc7f4d1c7);
  assert_eq!(out[5], 0x0368c033);
  assert_eq!(out[6], 0x9aaa2204);
  assert_eq!(out[7], 0x4e6cd4c3);
  assert_eq!(out[8], 0x466482d2);
  assert_eq!(out[9], 0x09aa9f07);
  assert_eq!(out[10], 0x05d7c214);
  assert_eq!(out[11], 0xa2028bd9);
  assert_eq!(out[12], 0xd19c12b5);
  assert_eq!(out[13], 0xb94e16de);
  assert_eq!(out[14], 0xe883d0cb);
  assert_eq!(out[15], 0x4e3c50a2);
}
}
