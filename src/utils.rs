use std::mem::{size_of};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub fn u64s_as_u8s(buf: &[u64]) -> &[u8] {
  let ptr = buf.as_ptr();
  let len = buf.len();
  let bytes_len = len * size_of::<u64>();
  unsafe { from_raw_parts(ptr as *const u8, bytes_len) }
}

pub fn u64s_as_u8s_mut(buf: &mut [u64]) -> &mut [u8] {
  let ptr = buf.as_mut_ptr();
  let len = buf.len();
  let bytes_len = len * size_of::<u64>();
  unsafe { from_raw_parts_mut(ptr as *mut u8, bytes_len) }
}
