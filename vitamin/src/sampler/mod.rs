pub use mixer::*;
pub use oscillator::*;

use crate::MessageReceiver;
use crate::network::Network;

mod mixer;
mod oscillator;

pub trait Sampler: MessageReceiver + Send {
  fn tick(&mut self, clock: f32, rate: f32);
  fn sample(&self, ctx: &Network, clock: f32, rate: f32) -> f32;
}
