extern crate rand;

use rand::{Rng, SeedableRng, thread_rng};
use std::fs::{File};
use std::io::{Read};
use std::marker::{PhantomData};
use std::path::{Path, PathBuf};

//pub mod categorical;
pub mod xorshift;

pub trait RngSeedExt {
  fn seed_size() -> usize;
}

pub trait RngState {
  fn state_size(&self) -> usize;
  fn extract_state(&self, state_buf: &mut [u64]);
  fn set_state(&mut self, state_buf: &[u64]);
}

pub trait RejectionSample {
  type Output;

  fn try_sample<R>(&mut self, rng: &mut R) -> Option<Self::Output> where R: Rng;
}

pub struct SeedFile<R> {
  seed_buf: Vec<u8>,
  _marker:  PhantomData<fn (R)>,
}

impl<R> Default for SeedFile<R> where R: RngSeedExt {
  fn default() -> Self {
    let default_path = PathBuf::from(".seed");
    match Self::open(&default_path) {
      Ok(seed) => seed,
      Err(_) => {
        match Self::create(&default_path, &mut thread_rng()) {
          Ok(seed) => seed,
          Err(_) => panic!(),
        }
      }
    }
  }
}

impl<R> SeedFile<R> where R: RngSeedExt {
  pub fn open(path: &Path) -> Result<SeedFile<R>, ()> {
    let mut file = match File::open(path) {
      Err(_) => return Err(()),
      Ok(file) => file,
    };
    let mut buf = Vec::with_capacity(R::seed_size());
    file.read_to_end(&mut buf);
    assert_eq!(buf.len(), R::seed_size());
    Ok(SeedFile{
      seed_buf: buf,
      _marker:  PhantomData,
    })
  }

  pub fn create<S>(path: &Path, src_rng: &mut S) -> Result<SeedFile<R>, ()> where S: Rng {
    unimplemented!();
  }
}
