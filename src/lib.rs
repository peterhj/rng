extern crate rand;

pub mod xorshift;

pub trait RngState {
  fn state_size(&self) -> usize;
  fn extract_state(&self, state_buf: &mut [u64]);
  fn set_state(&mut self, state_buf: &[u64]);
}
