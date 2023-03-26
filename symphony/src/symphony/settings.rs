pub(crate) struct Settings {
  pub(crate) volume: f32,
}

impl Default for Settings {
  fn default() -> Self {
    Self { volume: 0.0 }
  }
}
