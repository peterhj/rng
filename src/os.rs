pub use getrandom::{getrandom};

use std::io::{Read, Error as IoError};

#[derive(Default)]
pub struct GetrandomStream {
}

impl Read for GetrandomStream {
  #[inline]
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
    let buf_len = buf.len();
    getrandom(buf)?;
    Ok(buf_len)
  }
}
