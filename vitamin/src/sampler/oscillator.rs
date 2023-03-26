use std::f32::consts::TAU;

use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::{MessageReceiver, SmoothNum};
use crate::message::Message;
use crate::network::Network;
use crate::sampler::Sampler;

pub struct Oscillator {
  recv: Receiver<OscillatorMessage>,
  freq: SmoothNum,
}

pub enum OscillatorMessage {
  SetFreq(f32),
}

impl Oscillator {
  pub fn new(freq: f32) -> (Self, Sender<OscillatorMessage>) {
    let (send, recv) = unbounded();
    (Self { recv, freq: SmoothNum::new(freq, 1.5) }, send)
  }
}

impl Sampler for Oscillator {
  fn tick(&mut self, _clock: f32, _rate: f32) {
    self.freq.tick()
  }

  fn sample(&self, _ctx: &Network, clock: f32, rate: f32) -> f32 {
    (clock * self.freq.get() * TAU / rate).sin()
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
      OscillatorMessage::SetFreq(freq) => state.freq.set(freq),
    }
  }
}
