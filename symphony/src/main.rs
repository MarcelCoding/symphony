use std::thread;

use clap::Parser;
use eframe::egui;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use symphony::symphony::Symphony;
use symphony::window::SymphonyApp;

#[derive(Parser)]
#[clap(version)]
struct Args {}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .compact()
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  info!(concat!(
    "Booting ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "..."
  ));

  let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(1000.0, 800.0)),
    ..Default::default()
  };

  let symphony = Symphony::default();

  {
    let symphony = symphony.clone();
    thread::spawn(move || {
      symphony.play();
    });
  }

  eframe::run_native(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    options,
    Box::new(|_cc| Box::new(SymphonyApp::new(symphony))),
  )
    .unwrap();

  Ok(())
}
