extern crate byteorder;
extern crate libc;

use byteorder::{ReadBytesExt, NativeEndian};

use std::io::{Read, Error as IoError};
use std::mem::{size_of};
use std::ops::{RangeBounds, Bound};
use std::slice::{from_raw_parts};

pub mod romu;
pub mod splitmix;
pub mod urandom;
//pub mod xorshift;

pub trait Draw {
  type Item;

  fn draw<Rng: ReadBytesExt>(self, rng: Rng) -> Self::Item;
}

/*impl<R: RangeBounds<u32>> Draw for R {
  type Item = u32;

  fn draw<Rng: ReadBytesExt>(self, rng: Rng) -> u32 {
    draw_range_u32(self, rng)
  }
}*/

pub fn draw_range_u32<R: RangeBounds<u32>, Rng: ReadBytesExt>(r: R, rng: Rng) -> u32 {
  let lb = match r.start_bound() {
    Bound::Included(&lb) => lb,
    Bound::Excluded(&lb) => lb + 1,
    Bound::Unbounded => 0
  };
  let ub = match r.end_bound() {
    Bound::Included(&ub) => ub + 1,
    Bound::Excluded(&ub) => ub,
    Bound::Unbounded => panic!("bug")
  };
  lb + FastRangeU32::new(ub - lb).draw(rng)
}

impl<'a, T: Copy> Draw for &'a [T] {
  type Item = T;

  fn draw<Rng: ReadBytesExt>(self, rng: Rng) -> T {
    let mut r = FastRangeU32::new(self.len() as _);
    let i = r.draw(rng);
    self[i as usize]
  }
}

#[derive(Clone, Copy, Debug)]
pub struct FastRangeU32 {
  ub:   u32,
  cut:  u32,
}

impl Default for FastRangeU32 {
  #[inline]
  fn default() -> FastRangeU32 {
    FastRangeU32{ub: 0, cut: 0}
  }
}

impl FastRangeU32 {
  #[inline]
  pub fn new(ub: u32) -> FastRangeU32 {
    FastRangeU32{ub, cut: ub}
  }

  #[inline]
  pub fn upper_bound(&self) -> u32 {
    self.ub
  }

  #[inline]
  pub fn clear(&mut self) {
    self.ub = 0;
    self.cut = 0;
  }

  #[inline]
  pub fn reset(&mut self, ub: u32) {
    if self.ub != ub {
      self.ub = ub;
      self.cut = ub;
    }
  }

  pub fn draw<R: ReadBytesExt>(&mut self, mut rng: R) -> u32 {
    let s = self.ub;
    let mut x = rng.read_u32::<NativeEndian>().unwrap();
    let mut m = (x as u64).wrapping_mul(s as u64);
    let mut k = m as u32;
    if k < s {
      let t = if self.cut == s {
        let cut = s.wrapping_neg().wrapping_rem(s);
        self.cut = cut;
        cut
      } else {
        self.cut
      };
      while k < t {
        x = rng.read_u32::<NativeEndian>().unwrap();
        m = (x as u64).wrapping_mul(s as u64);
        k = m as u32;
      }
    }
    (m >> 32) as u32
  }
}

pub trait Generator<U> {
  fn next_gen(&mut self, out: &mut U);
}

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
    let x = (self.ubuf.as_ref()[self.cur / 4] >> (self.cur * 8)) as u8;
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

impl<R: From<u64>, U: AsRef<[u64]> + Default> From<u64> for Buffer64<R, U> {
  fn from(seed: u64) -> Buffer64<R, U> {
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
    let x = (self.ubuf.as_ref()[self.cur / 8] >> (self.cur * 8)) as u8;
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
