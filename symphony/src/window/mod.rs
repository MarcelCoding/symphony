use eframe::{App, Frame};
use eframe::egui::{Context, Window};

use crate::symphony::Symphony;
use crate::window::mixer::MixerWindow;
use crate::window::settings::SettingsWindow;

mod settings;
mod mixer;

pub struct SymphonyApp {
  settings: SettingsWindow,
  mixer: MixerWindow,
}

impl SymphonyApp {
  pub fn new(symphony: Symphony) -> Self {
    Self {
      settings: SettingsWindow::new(symphony.settings()),
      mixer: MixerWindow::new(symphony.mixer()),
    }
  }
}

impl App for SymphonyApp {
  fn update(&mut self, ctx: &Context, frame: &mut Frame) {
    Window::new("Settings").show(ctx, |ui| {
      self.settings.render(ui);
    });

    Window::new("Mixer").show(ctx, |ui| {
      self.mixer.render(ui);
    });
  }
}
