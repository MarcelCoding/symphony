use clap::Parser;
use eframe::egui;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use look::sampler::OscillatorWindow;
use look::WrappedNetwork;
use symphony::symphony::Symphony;

#[derive(Parser)]
#[clap(version)]
struct Args {}

fn main() -> anyhow::Result<()> {
  let _args = Args::parse();

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
    initial_window_size: Some(egui::vec2(1000.0, 600.0)),
    ..Default::default()
  };

  let (wrapped, network) = WrappedNetwork::new();

  let mut symphony = Symphony::new(wrapped);

  {
    let (id, oscillator, window) = OscillatorWindow::new();
    symphony.add_window(id, "A".to_string(), Box::new(oscillator), Box::new(window));

    let (id, oscillator, window) = OscillatorWindow::new();
    symphony.add_window(id, "B".to_string(), Box::new(oscillator), Box::new(window));

    let (id, oscillator, window) = OscillatorWindow::new();
    symphony.add_window(id, "C".to_string(), Box::new(oscillator), Box::new(window));
  }

  let _stream = symphony.play(network);

  eframe::run_native(
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    options,
    Box::new(|_cc| Box::new(symphony)),
  )
  .unwrap();

  Ok(())
}
