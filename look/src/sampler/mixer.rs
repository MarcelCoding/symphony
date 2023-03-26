use std::collections::HashMap;
use std::str::FromStr;

use crossbeam_channel::Sender;
use egui::{Context, Slider, Window};
use uuid::Uuid;

use vitamin::sampler::MixerMessage::AddSource;
use vitamin::sampler::{Mixer, MixerMessage};

use crate::sampler::SamplerWindow;
use crate::WrappedNetwork;

pub struct MixerWindow {
  send: Sender<MixerMessage>,
  id: Uuid,
  sources: HashMap<Uuid, f32>,
}

impl MixerWindow {
  pub fn new() -> (Uuid, Mixer, Self) {
    let id = Uuid::new_v4();
    let (mixer, send) = Mixer::new();
    (
      id,
      mixer,
      Self {
        id,
        send,
        sources: HashMap::new(),
      },
    )
  }
}

impl SamplerWindow for MixerWindow {
  fn render(&mut self, ctx: &Context, network: &WrappedNetwork) {
    let samplers = network.samplers.read().unwrap();

    for (id, name) in samplers.iter() {
      if id != &self.id && !self.sources.contains_key(id) && !name.contains("Channel") {
        self.sources.insert(*id, 0.0);
        self.send.send(AddSource(*id, 0.0)).unwrap();
      }
    }

    Window::new(format!(
      "Mixer {}",
      samplers.get(&self.id).unwrap().as_str()
    ))
    .show(ctx, |ui| {
      ui.label("Sources");

      for (id, volume) in &mut self.sources {
        ui.horizontal(|ui| {
          ui.label(samplers.get(id).unwrap());

          ui.add(
            Slider::from_get_set(0.0..=1.0, |val| {
              if let Some(val) = val {
                if &(val as f32) != volume {
                  *volume = val as f32;
                  self
                    .send
                    .try_send(MixerMessage::SetVolume(*id, *volume))
                    .unwrap();
                }
              }

              *volume as f64
            })
            .suffix("%")
            .custom_formatter(|val, _| format!("{}", (val * 100.0) as u8))
            .custom_parser(|val| u8::from_str(val).ok().map(|val| val as f64 / 100.0)),
          )
        });
      }
    });
  }
}
