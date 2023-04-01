use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Host, OutputCallbackInfo, SizedSample, Stream};
use eframe::egui::Context;
use eframe::{App, Frame};
use uuid::Uuid;

use look::sampler::{MixerWindow, SamplerWindow};
use look::WrappedNetwork;
use vitamin::sampler::Sampler;
use vitamin::Network;

pub struct Symphony {
  host: Host,
  network: WrappedNetwork,
  windows: Vec<Box<dyn SamplerWindow>>,
}

impl Symphony {
  pub fn new(network: WrappedNetwork) -> Self {
    Self {
      host: cpal::default_host(),
      network,
      windows: vec![],
    }
  }

  pub fn add_window(
    &mut self,
    id: Uuid,
    name: String,
    sampler: Box<dyn Sampler>,
    window: Box<dyn SamplerWindow>,
  ) {
    self.network.add_sampler(id, name, sampler);
    self.windows.push(window);
  }

  pub fn play(&mut self, network: Network) -> Stream {
    let device = self.host.default_output_device().unwrap();

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let channel_count = config.channels() as usize;
    let mut channels = Vec::with_capacity(channel_count);
    for i in 0..channel_count {
      let (id, mixer, window) = MixerWindow::new();

      channels.push(id);
      self
        .network
        .add_sampler(id, format!("Channel {}", i), Box::new(mixer));
      self.windows.push(Box::new(window));
    }

    match config.sample_format() {
      cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), channels, network),
      cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), channels, network),
      // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into(),x),
      cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), channels, network),
      // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into(),x),
      cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), channels, network),
      cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), channels, network),
      cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), channels, network),
      // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into(),x),
      cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), channels, network),
      // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into(),x),
      cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), channels, network),
      cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), channels, network),
      cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), channels, network),
      sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
    .unwrap()
  }
}

impl App for Symphony {
  fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
    self.network.recv();

    for window in &mut self.windows {
      window.render(ctx, &self.network);
    }
  }
}

pub fn run<T>(
  device: &cpal::Device,
  config: &cpal::StreamConfig,
  channels: Vec<Uuid>,
  mut network: Network,
) -> Result<Stream, anyhow::Error>
where
  T: SizedSample + FromSample<f32>,
{
  let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

  let sample_rate = config.sample_rate.0 as f32;
  let mut sample_clock = 0f32;

  let data_callback = move |data: &mut [T], _: &OutputCallbackInfo| {
    network.recv();

    for data in data.chunks_mut(channels.len()) {
      sample_clock = (sample_clock + 1.0) % sample_rate;
      network.tick(sample_clock, 1.0, sample_rate);

      for (i, item) in data.iter_mut().enumerate() {
        let sample = network.sample(&channels[i], sample_clock, 1.0, sample_rate);

        *item = T::from_sample(sample);
      }
    }
  };

  let stream = device.build_output_stream(config, data_callback, err_fn, None)?;

  stream.play()?;

  Ok(stream)
}
