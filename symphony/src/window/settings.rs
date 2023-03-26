use std::str::FromStr;
use std::sync::{Arc, Mutex};

use eframe::egui::{Slider, Ui};

use crate::symphony::settings::Settings;

pub(crate) struct SettingsWindow {
  settings: Arc<Mutex<Settings>>,
}

impl SettingsWindow {
  pub(crate) fn new(settings: Arc<Mutex<Settings>>) -> Self {
    Self { settings }
  }

  pub(crate) fn render(&mut self, ui: &mut Ui) {
    ui.horizontal(|ui| {
      ui.label("Volume");

      let mut settings = self.settings.lock().unwrap();

      ui.add(
        Slider::new(&mut settings.volume, 0.0..=1.0)
          .suffix("%")
          .custom_formatter(|val, _| format!("{}", (val * 100.0) as u8))
          .custom_parser(|val| u8::from_str(val).ok().map(|val| val as f64 / 100.0)),
      )
    });
  }
}
