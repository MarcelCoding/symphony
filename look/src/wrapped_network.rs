use std::collections::HashMap;
use std::sync::RwLock;

use crossbeam_channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

use vitamin::sampler::Sampler;
use vitamin::Network;

pub struct WrappedNetwork {
  send: Sender<(Uuid, String, Box<dyn Sampler>)>,
  recv: Receiver<(Uuid, String, Box<dyn Sampler>)>,
  delegate: Sender<(Uuid, Box<dyn Sampler>)>,
  pub samplers: RwLock<HashMap<Uuid, String>>,
}

impl WrappedNetwork {
  pub fn new() -> (Self, Network) {
    let (send, recv) = unbounded();
    let (network, delegate) = Network::new();

    (
      Self {
        send,
        recv,
        delegate,
        samplers: RwLock::new(HashMap::new()),
      },
      network,
    )
  }

  pub fn add_sampler(&self, id: Uuid, name: String, sampler: Box<dyn Sampler>) {
    self.send.try_send((id, name, sampler)).unwrap();
  }

  pub fn recv(&self) {
    while let Ok((id, name, sampler)) = self.recv.try_recv() {
      self.delegate.try_send((id, sampler)).unwrap();
      self.samplers.write().unwrap().insert(id, name);
    }
  }
}
