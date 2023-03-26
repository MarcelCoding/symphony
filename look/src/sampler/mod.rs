use egui::Context;

pub use mixer::*;
pub use oscillator::*;

use crate::WrappedNetwork;

mod mixer;
mod oscillator;

pub trait SamplerWindow {
  fn render(&mut self, ctx: &Context, network: &WrappedNetwork);
}
