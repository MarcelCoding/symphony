use crate::Sampler;

pub struct Input<S> {
  name: String,
  sample: S,
  volume: f32,
}

impl<S> Input<S> {
  pub fn name(&mut self) -> &mut String {
    &mut self.name
  }

  pub fn sampler(&mut self) -> &mut S {
    &mut self.sample
  }

  pub fn volume(&mut self) -> &mut f32 {
    &mut self.volume
  }
}

#[derive(Default)]
pub struct Mixer {
  sources: Vec<Input<Box<dyn Sampler + Send>>>,
}

impl Mixer {
  pub fn sources(&mut self) -> &mut Vec<Input<Box<dyn Sampler + Send>>> {
    &mut self.sources
  }

  pub fn add(&mut self, name: String, sample: Box<dyn Sampler + Send>) {
    self.sources.push(Input { name, sample, volume: 0.5 });
  }
}

impl Sampler for Mixer {
  fn sample(&mut self, clock: f32, rate: f32) -> f32 {
    let count = self.sources.len() as f32;

    let mut tone = 0.0;

    for Input { sample, volume, .. } in &mut self.sources {
      tone += sample.sample(clock, rate) * (*volume / count);
    }

    tone
  }
}