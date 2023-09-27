use crate::{Generator, Buffer32};

/* romu32x4_next:

Copyright 2020 Mark A. Overton

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License. */

pub fn romu32x4_next(state: &mut [u32; 4]) -> u32 {
  let [wp, xp, yp, zp] = *state;
  state[0] = 3323815723_u32.wrapping_mul(zp);
  state[1] = zp.wrapping_add(wp.rotate_left(26));
  state[2] = yp.wrapping_sub(xp);
  state[3] = (yp.wrapping_add(wp)).rotate_left(9);
  xp
}

pub struct Romu32x4Generator {
  state: [u32; 4],
}

impl From<[u32; 4]> for Romu32x4Generator {
  fn from(state: [u32; 4]) -> Romu32x4Generator {
    Romu32x4Generator{state}
  }
}

impl Generator<[u32; 1]> for Romu32x4Generator {
  #[inline]
  fn next_gen(&mut self, out: &mut [u32; 1]) {
    out[0] = romu32x4_next(&mut self.state);
  }
}

pub type Romu32x4Stream = Buffer32<Romu32x4Generator, [u32; 1]>;
