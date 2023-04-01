use crossbeam_channel::Sender;
use egui::{Context, Slider, Window};
use uuid::Uuid;

use vitamin::sampler::{Oscillator, OscillatorMessage};

use crate::sampler::SamplerWindow;
use crate::WrappedNetwork;

pub struct OscillatorWindow {
  send: Sender<OscillatorMessage>,
  id: Uuid,
  freq: f32,
}

impl OscillatorWindow {
  pub fn new() -> (Uuid, Oscillator, Self) {
    let id = Uuid::new_v4();
    let (oscillator, send) = Oscillator::new(440.0);
    (
      id,
      oscillator,
      Self {
        send,
        id,
        freq: 440.0,
      },
    )
  }
}

impl SamplerWindow for OscillatorWindow {
  fn render(&mut self, ctx: &Context, network: &WrappedNetwork) {
    let samplers = network.samplers.read().unwrap();

    Window::new(format!("Oscillator {}", samplers.get(&self.id).unwrap())).show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label("Frequency");

        ui.add(
          Slider::from_get_set(0.0..=5000.0, |val| {
            if let Some(val) = val {
              if (val as f32) != self.freq {
                self.freq = val as f32;
                self
                  .send
                  .send(OscillatorMessage::SetFreq(self.freq))
                  .unwrap();
              }
            }

            self.freq as f64
          })
          .suffix("Hz"),
        );
      });
    });
  }
}
