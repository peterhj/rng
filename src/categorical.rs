use rand::{Rng};
use std::u32;

pub struct AliasSampler32 {
  pr_alias: Vec<(f32, u32)>,
}

impl AliasSampler32 {
  pub fn new(probs: &[f32]) -> Self {
    let n = probs.len() as f32;
    let mut pr_alias = Vec::with_capacity(probs.len());
    let mut small = Vec::with_capacity(probs.len());
    let mut large = Vec::with_capacity(probs.len());
    for k in 0 .. probs.len() {
      pr_alias[k] = (0.0, u32::max_value());
      let u = probs[k] * n;
      if u < 1.0 {
        small.push((u, k as u32));
      } else {
        large.push((u, k as u32));
      }
    }
    let mut count = 0;
    while !small.is_empty() && !large.is_empty() {
      let (s_u, s_k) = small.pop().unwrap();
      let (l_u, l_k) = large.pop().unwrap();
      pr_alias[s_k as usize] = (probs[s_k as usize], l_k);
      let new_l_u = (s_u + l_u) - 1.0;
      if new_l_u < 1.0 {
        small.push((new_l_u, l_k));
      } else {
        large.push((new_l_u, l_k));
      }
      count += 1;
    }
    while !large.is_empty() {
      let (_, l_k) = large.pop().unwrap();
      pr_alias[l_k as usize] = (1.0, l_k);
      count += 1;
    }
    while !small.is_empty() {
      let (_, s_k) = small.pop().unwrap();
      pr_alias[s_k as usize] = (1.0, s_k);
      count += 1;
    }
    assert_eq!(count, probs.len());
    AliasSampler32{pr_alias}
  }

  pub fn sample<R>(&self, rng: &mut R) -> u32 where R: Rng {
    let k = rng.gen_range(0, self.pr_alias.len());
    let x = rng.gen::<f32>();
    let (p_k, a_k) = self.pr_alias[k];
    if x < p_k {
      k as _
    } else {
      a_k as _
    }
  }
}
