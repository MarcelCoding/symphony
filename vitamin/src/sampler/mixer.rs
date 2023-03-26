use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender, unbounded};
use uuid::Uuid;

use crate::{Message, MessageReceiver, SmoothNum};
use crate::network::Network;
use crate::sampler::Sampler;

pub struct Mixer {
  recv: Receiver<MixerMessage>,
  sources: HashMap<Uuid, SmoothNum>,
}

pub enum MixerMessage {
  AddSource(Uuid, f32),
  RemoveSource(Uuid),
  SetVolume(Uuid, f32),
}

impl Mixer {
  pub fn new() -> (Self, Sender<MixerMessage>) {
    let (send, recv) = unbounded();
    (
      Self {
        recv,
        sources: HashMap::new(),
      },
      send,
    )
  }
}

impl Sampler for Mixer {
  fn tick(&mut self, _clock: f32, _rate: f32) {
    for volume in self.sources.values_mut() {
      volume.tick();
    }
  }

  fn sample(&self, ctx: &Network, clock: f32, rate: f32) -> f32 {
    let mut count = 0.0;
    let mut tone = 0.0;

    for (id, volume) in &self.sources {
      if volume.get() == 0.0 {
        continue;
      }

      count += 1.0;
      tone += ctx.sample(id, clock, rate) * volume.get();
    }

    if count == 0.0 { 0.0 } else { tone / count }
  }
}

impl MessageReceiver for Mixer {
  fn recv(&mut self) {
    while let Ok(message) = self.recv.try_recv() {
      message.apply(self);
    }
  }
}

impl Message for MixerMessage {
  type State = Mixer;

  fn apply(self, state: &mut Self::State) {
    match self {
      MixerMessage::AddSource(id, volume) => {
        state.sources.insert(id, SmoothNum::new(volume, 0.0001));
      }
      MixerMessage::RemoveSource(id) => {
        state.sources.remove(&id);
      }
      MixerMessage::SetVolume(id, volume) => {
        if let Some(source) = state.sources.get_mut(&id) {
          source.set(volume);
        }
      }
    }
  }
}
