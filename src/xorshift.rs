use super::{RngState};
use utils::*;

use rand::{Rng};
use rand_core::{RngCore, SeedableRng, Error};
use std::num::{Wrapping};

#[derive(Clone)]
pub struct Xorshiftplus128Rng {
  state: [u64; 2],
}

impl Xorshiftplus128Rng {
  pub fn _state(&self) -> &[u64] {
    &self.state
  }
}

impl RngState for Xorshiftplus128Rng {
  fn state_size(&self) -> usize {
    2
  }

  fn extract_state(&self, state_buf: &mut [u64]) {
    state_buf[ .. 2].copy_from_slice(&self.state);
  }

  fn set_state(&mut self, state_buf: &[u64]) {
    self.state.copy_from_slice(&state_buf[ .. 2]);
  }
}

impl RngCore for Xorshiftplus128Rng {
  fn next_u32(&mut self) -> u32 {
    (self.next_u64() >> 32) as u32
  }

  fn next_u64(&mut self) -> u64 {
    let mut s1 = unsafe { *self.state.get_unchecked(0) };
    let s0 = unsafe { *self.state.get_unchecked(1) };
    s1 ^= s1 << 23;
    s1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
    unsafe { *self.state.get_unchecked_mut(0) = s0 };
    unsafe { *self.state.get_unchecked_mut(1) = s1 };
    (Wrapping(s1) + Wrapping(s0)).0
  }

  fn fill_bytes(&mut self, dst: &mut [u8]) {
    // TODO
    unimplemented!();
  }

  fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Error> {
    // TODO
    unimplemented!();
  }
}

impl SeedableRng for Xorshiftplus128Rng {
  type Seed = [u8; 16];

  fn from_seed(seed: [u8; 16]) -> Xorshiftplus128Rng {
    let mut rng = Xorshiftplus128Rng{
      state: [0; 2],
    };
    u64s_as_u8s_mut(&mut rng.state).copy_from_slice(&seed);
    rng
  }
}

#[derive(Clone)]
pub struct Xorshiftplus128v2Rng {
  state: [u64; 2],
}

impl RngCore for Xorshiftplus128v2Rng {
  fn next_u32(&mut self) -> u32 {
    (self.next_u64() >> 32) as u32
  }

  fn next_u64(&mut self) -> u64 {
    let mut s1 = unsafe { *self.state.get_unchecked(0) };
    let s0 = unsafe { *self.state.get_unchecked(1) };
    let r = (Wrapping(s1) + Wrapping(s0)).0;
    unsafe { *self.state.get_unchecked_mut(0) = s0 };
    s1 ^= s1 << 23;
    unsafe { *self.state.get_unchecked_mut(1) = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5) };
    r
  }

  fn fill_bytes(&mut self, dst: &mut [u8]) {
    // TODO
    unimplemented!();
  }

  fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Error> {
    // TODO
    unimplemented!();
  }
}

/*impl SeedableRng<[u64; 2]> for Xorshiftplus128Rng {
  fn reseed(&mut self, seed: [u64; 2]) {
    self.state[0] = seed[0];
    self.state[1] = seed[1];
    /*// XXX: This increases the initial state entropy (many zeros to half zeros).
    // See Figure 4 in <http://arxiv.org/abs/1404.0390> for details.
    for _ in 0 .. 20 {
      let _ = self.next_u64();
    }*/
  }

  fn from_seed(seed: [u64; 2]) -> Xorshiftplus128Rng {
    let mut rng = Xorshiftplus128Rng{
      state: [0; 2],
    };
    rng.reseed(seed);
    rng
  }
}

impl<'a> SeedableRng<&'a [u64]> for Xorshiftplus128Rng {
  fn reseed(&mut self, seed: &'a [u64]) {
    self.reseed([seed[0], seed[1]]);
  }

  fn from_seed(seed: &'a [u64]) -> Xorshiftplus128Rng {
    Self::from_seed([seed[0], seed[1]])
  }
}

impl<'a, R> SeedableRng<&'a mut R> for Xorshiftplus128Rng where R: Rng {
  fn reseed(&mut self, seed_rng: &'a mut R) {
    self.reseed([seed_rng.next_u64(), seed_rng.next_u64()]);
  }

  fn from_seed(seed_rng: &'a mut R) -> Xorshiftplus128Rng {
    Self::from_seed([seed_rng.next_u64(), seed_rng.next_u64()])
  }
}*/

/*pub struct Xorshiftstar1024Rng {
  state: [u64; 16],
  p: usize,
}

impl Rng for Xorshiftstar1024Rng {
  fn next_u64(&mut self) -> u64 {
    // See: <http://xorshift.di.unimi.it/xorshift1024star.c>
    // and <http://arxiv.org/abs/1402.6246>.
    let mut s0 = unsafe { *self.state.get_unchecked(self.p) };
    let p = (self.p + 1) & 0x0f;
    let mut s1 = unsafe { *self.state.get_unchecked(p) };
    s1 ^= s1 << 31;
    s1 ^= s1 >> 11;
    s0 ^= s0 >> 30;
    let s = s0 ^ s1;
    unsafe { *self.state.get_unchecked_mut(p) = s; }
    self.p = p;
    let r = s * 1181783497276652981_u64;
    r
  }

  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }
}

impl<'a> SeedableRng<&'a [u64]> for Xorshiftstar1024Rng {
  fn reseed(&mut self, seed: &'a [u64]) {
    assert!(seed.len() >= 16);
    for p in 0 .. 16 {
      self.state[p] = seed[p];
    }
  }

  fn from_seed(seed: &'a [u64]) -> Xorshiftstar1024Rng {
    let mut rng = Xorshiftstar1024Rng{
      state: [0; 16],
      p: 0,
    };
    rng.reseed(seed);
    rng
  }
}*/
