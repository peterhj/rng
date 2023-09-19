use crate::{Generator, Buffer64};

/* splitmix64_next:

Written in 2015 by Sebastiano Vigna (vigna@acm.org)

To the extent possible under law, the author has dedicated all copyright
and related and neighboring rights to this software to the public domain
worldwide. This software is distributed without any warranty.

See <http://creativecommons.org/publicdomain/zero/1.0/>. */

pub fn splitmix64_next(state: &mut u64) -> u64 {
  let mut x = *state;
  x = x.wrapping_add(0x9e3779b97f4a7c15);
  let mut z = x;
  *state = x;
  z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
  z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
  z ^ (z >> 31)
}

pub struct Splitmix64Generator {
  state: u64,
}

impl From<u64> for Splitmix64Generator {
  fn from(seed: u64) -> Splitmix64Generator {
    Splitmix64Generator{state: seed}
  }
}

impl Generator<[u64; 1]> for Splitmix64Generator {
  fn next_gen(&mut self, out: &mut [u64; 1]) {
    out[0] = splitmix64_next(&mut self.state);
  }
}

pub type Splitmix64Stream = Buffer64<Splitmix64Generator, [u64; 1]>;
