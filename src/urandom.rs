#[cfg(target_os = "linux")]
use libc::{c_void, getrandom};

#[cfg(target_os = "linux")]
pub fn urandom_gen(buf: &mut [u8]) -> Result<(), ()> {
  let mut off = 0;
  unsafe {
    while (off as usize) < buf.len() {
      let ret = getrandom(buf.as_mut_ptr().offset(off) as *mut c_void, buf.len() - (off as usize), 0);
      if ret < 0 {
        return Err(());
      }
      off += ret;
    }
  }
  Ok(())
}
