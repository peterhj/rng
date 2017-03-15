extern crate rand;

use rand::{Rng};

//pub mod categorical;
pub mod xorshift;

pub trait RngState {
  fn state_size(&self) -> usize;
  fn extract_state(&self, state_buf: &mut [u64]);
  fn set_state(&mut self, state_buf: &[u64]);
}

pub trait RejectionSampler<Output> {
  fn try_sample<R>(&mut self, rng: &mut R) -> Option<Output> where R: Rng;
}
