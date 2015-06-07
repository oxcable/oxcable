//! Provides audio IO from OS sound devices.

extern crate portaudio;

use std::rc::Rc;

use types::{SAMPLE_RATE, AudioDevice, DeviceIOType, Sample, Time};


/// Defines the audio format for Portaudio.
static PORTAUDIO_T: portaudio::pa::SampleFormat =
    portaudio::pa::SampleFormat::Float32;

/// Defines the buffer size for Portaudio
static BUFFER_SIZE: usize = 256;


/// Used to handle portaudio resources.
pub struct AudioEngine;

impl AudioEngine {
    pub fn open() -> Result<AudioEngine, String> {
        match portaudio::pa::initialize() {
            Ok(()) => Ok(AudioEngine),
            Err(e) => Err(portaudio::pa::get_error_text(e))
        }
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self)
    {
        assert!(portaudio::pa::terminate().is_ok());
    }
}


/// Reads audio from the OS's default input device.
pub struct AudioIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<AudioEngine>,
    pa_stream: portaudio::pa::Stream<Sample, Sample>,
    num_channels: usize,
    buffer: Vec<Sample>,
    samples_read: usize,
}

impl AudioIn {
    /// Opens an audio input stream reading `num_channels` inputs.
    pub fn new(engine: Rc<AudioEngine>, num_channels: usize) -> AudioIn {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        assert!(pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                                       num_channels as i32, 0i32,
                                       PORTAUDIO_T, None).is_ok());
        assert!(pa_stream.start().is_ok());

        AudioIn {
            engine: engine,
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_read: BUFFER_SIZE,
        }
    }
}

impl Drop for AudioIn {
    fn drop(&mut self) {
        assert!(self.pa_stream.stop().is_ok());
        assert!(self.pa_stream.close().is_ok());
    }
}

impl AudioDevice for AudioIn {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(0)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn tick(&mut self, _: Time, _: &[Sample], outputs: &mut[Sample]) {
        if self.samples_read == BUFFER_SIZE {
            let result = self.pa_stream.read(BUFFER_SIZE as u32);
            match result {
                Ok(v) => self.buffer = v.clone(),
                Err(e) => panic!(e)
            }
            self.samples_read = 0;
        }

        for i in (0 .. self.num_channels) {
            outputs[i] = self.buffer[self.samples_read*self.num_channels + i];
        }
        self.samples_read += 1;
    }
}


/// Writes audio to the OS's default output device.
pub struct AudioOut {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<AudioEngine>,
    pa_stream: portaudio::pa::Stream<Sample, Sample>,
    num_channels: usize,
    buffer: Vec<Sample>,
    samples_written: usize,
}

impl AudioOut {
    /// Opens an output stream writing `num_channels` outputs.
    pub fn new(engine: Rc<AudioEngine>, num_channels: usize) -> AudioOut {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        assert!(pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                                       0i32, num_channels as i32,
                                       PORTAUDIO_T, None).is_ok());
        assert!(pa_stream.start().is_ok());

        AudioOut {
            engine: engine,
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_written: 0,
        }
    }
}

impl Drop for AudioOut {
    fn drop(&mut self) {
        assert!(self.pa_stream.stop().is_ok());
        assert!(self.pa_stream.close().is_ok());
    }
}

impl AudioDevice for AudioOut {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(0)
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], _: &mut[Sample]) {
        for s in inputs.iter() {
            let mut clipped = *s;
            if clipped > 1.0 { clipped = 1.0; }
            if clipped < -1.0 { clipped = -1.0; }
            self.buffer.push(clipped)
        }
        self.samples_written += 1;

        if self.samples_written == BUFFER_SIZE {
            assert!(self.pa_stream.write(self.buffer.clone(),
                                         BUFFER_SIZE as u32).is_ok());
            self.samples_written = 0;
            self.buffer.clear()
        }
    }
}
