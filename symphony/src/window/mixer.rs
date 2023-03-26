use std::str::FromStr;
use std::sync::{Arc, Mutex};

use eframe::egui::{Slider, Ui};

use vitamin::mixer::Mixer;

pub(crate) struct MixerWindow {
  mixer: Arc<Mutex<Mixer>>,
}

impl MixerWindow {
  pub(crate) fn new(mixer: Arc<Mutex<Mixer>>) -> Self {
    Self { mixer }
  }

  pub(crate) fn render(&mut self, ui: &mut Ui) {
    ui.label("Sources");

    let mut mixer = self.mixer.lock().unwrap();

    for input in mixer.sources() {
      ui.horizontal(|ui| {
        ui.label(input.name().to_string());

        ui.add(
          Slider::new(input.volume(), 0.0..=1.0)
            .suffix("%")
            .custom_formatter(|val, _| format!("{}", (val * 100.0) as u8))
            .custom_parser(|val| u8::from_str(val).ok().map(|val| val as f64 / 100.0)),
        )
      });
    }
  }
}
