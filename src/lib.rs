extern crate byteorder;
extern crate libc;

use std::io::{Read, Error as IoError};
use std::mem::{size_of};
use std::slice::{from_raw_parts};

pub mod chacha20;
pub mod dist;
pub mod romu;
pub mod splitmix;
pub mod urandom;
pub mod xorshift;

pub trait Generator<U> {
  fn next_gen(&mut self, out: &mut U);
}

/*impl<G: Generator<[u32; 1]>> Iterator for G {
  type Item = u32;

  #[inline]
  fn next(&mut self) -> Option<u32> {
    let mut out = [0];
    self.next_gen(&mut out);
    Some(out[0])
  }
}

impl<G: Generator<[u64; 1]>> Iterator for G {
  type Item = u64;

  #[inline]
  fn next(&mut self) -> Option<u64> {
    let mut out = [0];
    self.next_gen(&mut out);
    Some(out[0])
  }
}*/

/*pub trait Stream {
  fn next_u8(&mut self) -> u8;

  fn next_u16(&mut self) -> u16 {
    u16::from_le_bytes([
         self.next_u8()
        ,self.next_u8()
    ])
  }

  fn next_u32(&mut self) -> u32 {
    u32::from_le_bytes([
         self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
    ])
  }

  fn next_u64(&mut self) -> u64 {
    u64::from_le_bytes([
         self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
        ,self.next_u8()
    ])
  }
}*/

#[inline]
pub fn u32_slice_bytes_len(ubuf: &[u32]) -> usize {
  ubuf.len() * size_of::<u32>()
}

#[inline]
pub fn u32_slice_as_bytes(ubuf: &[u32]) -> &[u8] {
  unsafe { from_raw_parts(ubuf.as_ptr() as *const _, ubuf.len() * size_of::<u32>()) }
}

#[inline]
pub fn u64_slice_bytes_len(ubuf: &[u64]) -> usize {
  ubuf.len() * size_of::<u64>()
}

#[inline]
pub fn u64_slice_as_bytes(ubuf: &[u64]) -> &[u8] {
  unsafe { from_raw_parts(ubuf.as_ptr() as *const _, ubuf.len() * size_of::<u64>()) }
}

pub struct Buffer32<R, U> {
  gen:  R,
  ubuf: U,
  cur:  usize,
}

impl<R, U: AsRef<[u32]> + Default> Buffer32<R, U> {
  pub fn new(gen: R) -> Buffer32<R, U> {
    let ubuf = U::default();
    let cur = u32_slice_bytes_len(ubuf.as_ref());
    Buffer32{gen, ubuf, cur}
  }
}

impl<R, U: AsRef<[u32]> + Default> Buffer32<R, U> {
  pub fn from<S>(seed: S) -> Buffer32<R, U> where R: From<S> {
    let gen = R::from(seed);
    let ubuf = U::default();
    let cur = u32_slice_bytes_len(ubuf.as_ref());
    Buffer32{gen, ubuf, cur}
  }
}

impl<R: Generator<U>, U: AsRef<[u32]>> Iterator for Buffer32<R, U> {
  type Item = u8;

  #[inline]
  fn next(&mut self) -> Option<u8> {
    let len = u32_slice_bytes_len(self.ubuf.as_ref());
    if self.cur >= len {
      self.gen.next_gen(&mut self.ubuf);
      self.cur = 1;
      return Some(self.ubuf.as_ref()[0] as u8);
    }
    let x = (self.ubuf.as_ref()[self.cur / 4] >> ((self.cur % 4) * 8)) as u8;
    self.cur += 1;
    Some(x)
  }
}

impl<R: Generator<U>, U: AsRef<[u32]>> Read for Buffer32<R, U> {
  #[inline]
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
    for x in buf.iter_mut() {
      *x = self.next().unwrap();
    }
    Ok(buf.len())
  }
}

pub struct Buffer64<R, U> {
  gen:  R,
  ubuf: U,
  cur:  usize,
}

impl<R, U: AsRef<[u64]> + Default> Buffer64<R, U> {
  pub fn new(gen: R) -> Buffer64<R, U> {
    let ubuf = U::default();
    let cur = u64_slice_bytes_len(ubuf.as_ref());
    Buffer64{gen, ubuf, cur}
  }
}

impl<R, U: AsRef<[u64]> + Default> Buffer64<R, U> {
  pub fn from<S>(seed: S) -> Buffer64<R, U> where R: From<S> {
    let gen = R::from(seed);
    let ubuf = U::default();
    let cur = u64_slice_bytes_len(ubuf.as_ref());
    Buffer64{gen, ubuf, cur}
  }
}

impl<R: Generator<U>, U: AsRef<[u64]>> Iterator for Buffer64<R, U> {
  type Item = u8;

  #[inline]
  fn next(&mut self) -> Option<u8> {
    let len = u64_slice_bytes_len(self.ubuf.as_ref());
    if self.cur >= len {
      self.gen.next_gen(&mut self.ubuf);
      self.cur = 1;
      return Some(self.ubuf.as_ref()[0] as u8);
    }
    let x = (self.ubuf.as_ref()[self.cur / 8] >> ((self.cur % 8) * 8)) as u8;
    self.cur += 1;
    Some(x)
  }
}

impl<R: Generator<U>, U: AsRef<[u64]>> Read for Buffer64<R, U> {
  #[inline]
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
    for x in buf.iter_mut() {
      *x = self.next().unwrap();
    }
    Ok(buf.len())
  }
}
