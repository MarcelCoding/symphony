pub use message::*;
pub use network::*;

mod message;
mod network;
pub mod sampler;

#[cfg(test)]
mod tests {
  use crossbeam_channel::unbounded;

  #[test]
  fn test() {
    let (send, recv) = unbounded();
    send.try_send(4).unwrap();

    println!("{:?}", recv.try_recv().unwrap());
  }
}

pub(crate) struct SmoothNum {
  curr: f32,
  target: f32,
  step: f32,
}

impl SmoothNum {
  pub(crate) fn new(val: f32, step: f32) -> Self {
    Self {
      curr: val,
      target: val,
      step,
    }
  }

  pub(crate) fn tick(&mut self) {
    if self.curr == self.target {
      return;
    }

    let delta = (self.target - self.curr).abs();
    let step = self.step.min(delta);

    if self.curr < self.target {
      self.curr += step;
    } else {
      self.curr -= step;
    }
  }

  pub(crate) fn set(&mut self, new: f32) {
    self.target = new
  }

  pub(crate) fn get(&self) -> f32 {
    self.curr
  }
}
