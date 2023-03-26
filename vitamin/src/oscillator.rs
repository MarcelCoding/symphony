use std::f32::consts::TAU;
use crate::Sampler;

pub struct Oscillator {
  freq: f32,
}

impl Oscillator {
  pub fn new(freq: f32) -> Self {
    Self { freq }
  }
}

impl Sampler for Oscillator {
  fn sample(&mut self, clock: f32, rate: f32) -> f32 {
    (clock * self.freq * TAU / rate).sin()
  }
}