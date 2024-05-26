use std::cmp::{min};
use std::io::{Read, Error as IoError};

#[inline]
pub fn getrandom(buf: &mut [u8]) -> Result<(), IoError> {
  RandomStream{}.read(buf).map(|_| ())
}

pub type GetrandomStream = RandomStream;

#[derive(Default)]
pub struct RandomStream {
}

#[cfg(target_os = "linux")]
impl Read for RandomStream {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
    const CHUNK_CAP: usize = 256;
    let buf_len = buf.len();
    let mut o = 0;
    while o < buf_len {
      let chunk_len = min(buf_len - o, CHUNK_CAP);
      let o2 = o + chunk_len;
      let mut chunk = &mut buf[o .. o2];
      let ret = unsafe { libc::getrandom(chunk.as_mut_ptr() as *mut _, chunk_len, 0) };
      // FIXME: may want to allow returning partial result.
      if ret < 0 {
        return Err(IoError::last_os_error());
      } else {
        assert_eq!(ret, chunk_len);
      }
      o = o2;
    }
    Ok(buf_len)
  }
}

#[cfg(target_os = "macos")]
impl Read for RandomStream {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
    const CHUNK_CAP: usize = 256;
    let buf_len = buf.len();
    let mut o = 0;
    while o < buf_len {
      let chunk_len = min(buf_len - o, CHUNK_CAP);
      let o2 = o + chunk_len;
      let mut chunk = &mut buf[o .. o2];
      let ret = unsafe { libc::getentropy(chunk.as_mut_ptr() as *mut _, chunk_len) };
      // FIXME: may want to allow returning partial result.
      if ret != 0 {
        return Err(IoError::last_os_error());
      }
      o = o2;
    }
    Ok(buf_len)
  }
}
