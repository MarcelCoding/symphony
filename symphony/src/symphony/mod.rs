use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cpal::{FromSample, Host, Sample, SizedSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tracing::info;

use vitamin::mixer::Mixer;
use vitamin::oscillator::Oscillator;
use vitamin::Sampler;

use crate::symphony::settings::Settings;

pub(crate) mod settings;

#[derive(Clone)]
pub struct Symphony {
  host: Arc<Mutex<Host>>,
  settings: Arc<Mutex<Settings>>,
  mixer: Arc<Mutex<Mixer>>,
}

impl Symphony {
  pub(crate) fn settings(&self) -> Arc<Mutex<Settings>> {
    self.settings.clone()
  }

  pub(crate) fn mixer(&self) -> Arc<Mutex<Mixer>> {
    self.mixer.clone()
  }

  pub fn play(&self) {
    let symphony = self.clone();
    symphony.mixer.lock().unwrap().add("440 Hz".to_string(),Box::new(Oscillator::new(440.0)));
    symphony.mixer.lock().unwrap().add("300 Hz".to_string(),Box::new(Oscillator::new(300.0)));
    symphony.mixer.lock().unwrap().add("600 Hz".to_string(),Box::new(Oscillator::new(600.0)));

    thread::spawn(move || {
      let device = symphony.host.lock().unwrap().default_output_device().unwrap();

      let config = device.default_output_config().unwrap();
      println!("Default output config: {:?}", config);

      let x = move |clock, rate| symphony.mixer.lock().unwrap().sample(clock, rate) * symphony.settings.lock().unwrap().volume;

      match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8, _>(&device, &config.into(), x),
        cpal::SampleFormat::I16 => run::<i16, _>(&device, &config.into(), x),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into(),x),
        cpal::SampleFormat::I32 => run::<i32, _>(&device, &config.into(), x),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into(),x),
        cpal::SampleFormat::I64 => run::<i64, _>(&device, &config.into(), x),
        cpal::SampleFormat::U8 => run::<u8, _>(&device, &config.into(), x),
        cpal::SampleFormat::U16 => run::<u16, _>(&device, &config.into(), x),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into(),x),
        cpal::SampleFormat::U32 => run::<u32, _>(&device, &config.into(), x),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into(),x),
        cpal::SampleFormat::U64 => run::<u64, _>(&device, &config.into(), x),
        cpal::SampleFormat::F32 => run::<f32, _>(&device, &config.into(), x),
        cpal::SampleFormat::F64 => run::<f64, _>(&device, &config.into(), x),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
      }.unwrap();
    });
  }
}

impl Default for Symphony {
  fn default() -> Self {
    let host = cpal::default_host();

    Self {
      host: Arc::new(Mutex::new(host)),
      settings: Default::default(),
      mixer: Default::default(),
    }
  }
}

pub fn run<T, F: FnMut(f32, f32) -> f32 + Send + 'static>(device: &cpal::Device, config: &cpal::StreamConfig, mut next_sample: F) -> Result<(), anyhow::Error>
  where
    T: SizedSample + FromSample<f32>,
{
  let sample_rate = config.sample_rate.0 as f32;
  let channels = config.channels as usize;

  // Produce a sinusoid of maximum amplitude.
  let mut sample_clock = 0f32;
  let mut next_value = move || {
    sample_clock = (sample_clock + 1.0) % sample_rate;
    next_sample(sample_clock, sample_rate)
  };

  let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

  info!("play");

  let stream = device.build_output_stream(
    config,
    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
      write_data(data, channels, &mut next_value)
    },
    err_fn,
    None,
  )?;


  stream.play()?;

  thread::sleep(Duration::from_secs(9999999));

  Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
  where
    T: Sample + FromSample<f32>,
{
  for frame in output.chunks_mut(channels) {
    let value: T = T::from_sample(next_sample());
    for sample in frame.iter_mut() {
      *sample = value;
    }
  }
}
