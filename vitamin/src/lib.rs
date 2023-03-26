pub mod mixer;
pub mod oscillator;

pub trait Sampler {
  fn sample(&mut self, clock: f32, rate: f32) -> f32;
}