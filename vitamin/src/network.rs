use std::collections::HashMap;

use crossbeam_channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

use crate::sampler::Sampler;

pub struct Network {
  recv: Receiver<(Uuid, Box<dyn Sampler>)>,
  samplers: HashMap<Uuid, Box<dyn Sampler>>,
}

impl Network {
  pub fn new() -> (Self, Sender<(Uuid, Box<dyn Sampler>)>) {
    let (send, recv) = unbounded();
    (
      Self {
        recv,
        samplers: HashMap::new(),
      },
      send,
    )
  }

  pub fn recv(&mut self) {
    while let Ok((id, sampler)) = self.recv.try_recv() {
      self.samplers.insert(id, sampler);
    }

    for sampler in self.samplers.values_mut() {
      sampler.recv();
    }
  }

  pub fn tick(&mut self, clock: f32, clock_delta: f32, rate: f32) {
    for sampler in self.samplers.values_mut() {
      sampler.tick(clock, clock_delta, rate);
    }
  }

  pub fn sample(&self, id: &Uuid, clock: f32, clock_delta: f32, rate: f32) -> f32 {
    match self.samplers.get(id) {
      Some(sampler) => sampler.sample(self, clock, clock_delta, rate),
      None => 0.0,
    }
  }
}
