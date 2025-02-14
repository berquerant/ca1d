use chrono::Utc;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;

pub struct BoolRng(StdRng);

fn seed_u64_from_now() -> u64 {
    Utc::now().timestamp() as u64
}

impl BoolRng {
    pub fn new(seed: Option<u64>) -> Self {
        let r = StdRng::seed_from_u64(seed.unwrap_or(seed_u64_from_now()));
        BoolRng(r)
    }
    pub fn gen_bool(&mut self) -> bool {
        let v: f64 = self.0.random();
        v < 0.5
    }
    pub fn gen_bools(&mut self, n: usize) -> Vec<bool> {
        let mut v = vec![false; n];
        for x in v.iter_mut() {
            *x = self.gen_bool();
        }
        v
    }
}
