use std::f32::consts::TAU;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::message::Message;
use crate::network::Network;
use crate::sampler::Sampler;
use crate::MessageReceiver;

pub struct Oscillator {
  recv: Receiver<OscillatorMessage>,
  freq: f32,
  last_val: f32,
}

pub enum OscillatorMessage {
  SetFreq(f32),
}

impl Oscillator {
  pub fn new(freq: f32) -> (Self, Sender<OscillatorMessage>) {
    let (send, recv) = unbounded();
    (
      Self {
        recv,
        freq,
        last_val: 0.0,
      },
      send,
    )
  }
}

impl Sampler for Oscillator {
  fn tick(&mut self, _clock: f32, clock_delta: f32, rate: f32) {
    self.last_val += clock_delta * self.freq * TAU / rate;
  }

  fn sample(&self, _ctx: &Network, _clock: f32, _clock_delta: f32, _rate: f32) -> f32 {
    self.last_val.sin()
  }
}

impl MessageReceiver for Oscillator {
  fn recv(&mut self) {
    while let Ok(message) = self.recv.try_recv() {
      message.apply(self);
    }
  }
}

impl Message for OscillatorMessage {
  type State = Oscillator;

  fn apply(self, state: &mut Self::State) {
    match self {
      OscillatorMessage::SetFreq(freq) => state.freq = freq,
    }
  }
}
