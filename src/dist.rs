use byteorder::{ReadBytesExt, LittleEndian as LE};

use std::io::{Read};
use std::ops::{RangeBounds, Bound};

pub trait Draw {
  type Item;

  fn draw<Rng: Read>(self, rng: Rng) -> Self::Item;
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
    let mut x = rng.read_u32::<LE>().unwrap();
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
        x = rng.read_u32::<LE>().unwrap();
        m = (x as u64).wrapping_mul(s as u64);
        k = m as u32;
      }
    }
    (m >> 32) as u32
  }
}

/*impl<R: RangeBounds<u32>> Draw for R {
  type Item = u32;

  fn draw<Rng: Read>(self, rng: Rng) -> u32 {
    draw_range_u32(self, rng)
  }
}*/

pub fn draw_range_u8<R: RangeBounds<u8>, Rng: Read>(r: R, rng: Rng) -> u8 {
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
  lb + (FastRangeU32::new((ub - lb) as _).draw(rng) as u8)
}

pub fn draw_range_u32<R: RangeBounds<u32>, Rng: Read>(r: R, rng: Rng) -> u32 {
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

  fn draw<Rng: Read>(self, rng: Rng) -> T {
    let mut r = FastRangeU32::new(self.len() as _);
    let i = r.draw(rng);
    self[i as usize]
  }
}

pub fn shuffle<S: AsMut<[T]>, T, R: ReadBytesExt>(mut buf: S, mut rng: R) {
  let mut buf = buf.as_mut();
  if buf.len() <= 1 {
    return;
  }
  assert!(buf.len() <= u32::max_value() as usize);
  let len = buf.len() as u32;
  let mut r = FastRangeU32::default();
  for off in 0 .. len - 1 {
    r.reset(len - off);
    let i = r.draw(&mut rng);
    if i != 0 {
      buf.swap(off as usize, (off + i) as usize);
    }
  }
}
