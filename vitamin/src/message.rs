pub trait MessageReceiver {
  fn recv(&mut self);
}

pub trait Message {
  type State;

  fn apply(self, state: &mut Self::State);
}
