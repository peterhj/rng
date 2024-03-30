use crate::{Generator, Buffer64};

use byteorder::{ReadBytesExt, LittleEndian as LE};

use std::io::{Read};

/* xoroshiro1024_next, xorshiftplus128v1_next, xorshiftplus128v2_next:

Written in _ by Sebastiano Vigna (vigna@acm.org)

To the extent possible under law, the author has dedicated all copyright
and related and neighboring rights to this software to the public domain
worldwide. This software is distributed without any warranty.

See <http://creativecommons.org/publicdomain/zero/1.0/>. */

// This is the xoroshiro1024++ variant.
pub fn xoroshiro1024_next(state: &mut [u64; 16], cursor: &mut u8) -> u64 {
  let q = *cursor;
  let np = q.wrapping_add(1) & 15;
  let s15 = state[q as usize];
  let s0 = state[np as usize];
  let out = ((s0.wrapping_add(s15)).rotate_left(23)).wrapping_add(s15);
  let s15 = s0 ^ s15;
  state[q as usize] = (s0.rotate_left(25)) ^ s15 ^ (s15.rotate_left(27));
  state[np as usize] = s15.rotate_left(36);
  *cursor = np;
  out
}

pub struct Xoroshiro1024Generator {
  state: [u64; 16],
  cursor: u8,
}

impl From<[u64; 16]> for Xoroshiro1024Generator {
  fn from(state: [u64; 16]) -> Xoroshiro1024Generator {
    Xoroshiro1024Generator{state, cursor: 0}
  }
}

impl<'r> From<&'r mut dyn Read> for Xoroshiro1024Generator {
  fn from(reader: &'r mut dyn Read) -> Xoroshiro1024Generator {
    let mut state = [0; 16];
    for k in 0 .. 16 {
      state[k] = reader.read_u64::<LE>().unwrap();
    }
    Xoroshiro1024Generator{state, cursor: 0}
  }
}

impl Generator<[u64; 1]> for Xoroshiro1024Generator {
  #[inline]
  fn next_gen(&mut self, out: &mut [u64; 1]) {
    out[0] = xoroshiro1024_next(&mut self.state, &mut self.cursor);
  }
}

pub type Xoroshiro1024Stream = Buffer64<Xoroshiro1024Generator, [u64; 1]>;

pub fn xorshiftplus128v1_next(state: &mut [u64; 2]) -> u64 {
  let mut s1 = state[0];
  let s0 = state[1];
  s1 ^= s1 << 23;
  s1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
  state[0] = s0;
  state[1] = s1;
  s1.wrapping_add(s0)
}

pub fn xorshiftplus128v2_next(state: &mut [u64; 2]) -> u64 {
  let mut s1 = state[0];
  let s0 = state[1];
  let r = s1.wrapping_add(s0);
  state[0] = s0;
  s1 ^= s1 << 23;
  state[1] = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5);
  r
}

pub struct Xorshiftplus128v1Generator {
  state: [u64; 2],
}

impl From<[u64; 2]> for Xorshiftplus128v1Generator {
  fn from(state: [u64; 2]) -> Xorshiftplus128v1Generator {
    Xorshiftplus128v1Generator{state}
  }
}

impl Generator<[u64; 1]> for Xorshiftplus128v1Generator {
  #[inline]
  fn next_gen(&mut self, out: &mut [u64; 1]) {
    out[0] = xorshiftplus128v1_next(&mut self.state);
  }
}

pub struct Xorshiftplus128v2Generator {
  state: [u64; 2],
}

impl From<[u64; 2]> for Xorshiftplus128v2Generator {
  fn from(state: [u64; 2]) -> Xorshiftplus128v2Generator {
    Xorshiftplus128v2Generator{state}
  }
}

impl Generator<[u64; 1]> for Xorshiftplus128v2Generator {
  #[inline]
  fn next_gen(&mut self, out: &mut [u64; 1]) {
    out[0] = xorshiftplus128v2_next(&mut self.state);
  }
}
