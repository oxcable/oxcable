//! Provides audio IO from OS sound devices.

extern crate portaudio;

use std::rc::Rc;

use types::{SAMPLE_RATE, AudioDevice, Sample, Time};


/// Defines the audio format for Portaudio.
static PORTAUDIO_T: portaudio::pa::SampleFormat =
    portaudio::pa::SampleFormat::Float32;

/// Defines the buffer size for Portaudio
static BUFFER_SIZE: usize = 256;


/// This empty struct is used as a RAII marker for an initialized portaudio
/// connection. It is held in a Rc, and copies are passed to all streams opened
/// with it.
struct AudioEngineMarker;
impl Drop for AudioEngineMarker {
    fn drop(&mut self)
    {
        portaudio::pa::terminate().unwrap();
    }
}

/// The AudioEnginer opens and manages the resources associated with portaudio.
/// It is used open new input/output streams and safely free them.
pub struct AudioEngine {
    marker: Rc<AudioEngineMarker>
}

impl AudioEngine {
    pub fn open() -> Result<AudioEngine, String> {
        match portaudio::pa::initialize() {
            Ok(()) => Ok(AudioEngine { marker: Rc::new(AudioEngineMarker) }),
            Err(e) => Err(portaudio::pa::get_error_text(e))
        }
    }

    pub fn default_input(&self, num_channels: usize) -> AudioIn {
        AudioIn::new(self.marker.clone(), num_channels)
    }

    pub fn default_output(&self, num_channels: usize) -> AudioOut {
        AudioOut::new(self.marker.clone(), num_channels)
    }
}


/// Reads audio from the OS's default input device.
pub struct AudioIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<AudioEngineMarker>,
    pa_stream: portaudio::pa::Stream<Sample, Sample>,
    num_channels: usize,
    buffer: Vec<Sample>,
    samples_read: usize,
}

impl AudioIn {
    /// Opens an audio input stream reading `num_channels` inputs.
    fn new(engine: Rc<AudioEngineMarker>, num_channels: usize) -> AudioIn {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                               num_channels as i32, 0i32,
                               PORTAUDIO_T, None).unwrap();
        pa_stream.start().unwrap();

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
        self.pa_stream.stop().unwrap();
        self.pa_stream.close().unwrap();
    }
}

impl AudioDevice for AudioIn {
    fn num_inputs(&self) -> usize {
        0
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
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
    engine: Rc<AudioEngineMarker>,
    pa_stream: portaudio::pa::Stream<Sample, Sample>,
    num_channels: usize,
    buffer: Vec<Sample>,
    samples_written: usize,
}

impl AudioOut {
    /// Opens an output stream writing `num_channels` outputs.
    fn new(engine: Rc<AudioEngineMarker>, num_channels: usize) -> AudioOut {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                               0i32, num_channels as i32,
                               PORTAUDIO_T, None).unwrap();
        pa_stream.start().unwrap();

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
        self.pa_stream.stop().unwrap();
        self.pa_stream.close().unwrap();
    }
}

impl AudioDevice for AudioOut {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        0
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
            self.pa_stream.write(self.buffer.clone(),
                                 BUFFER_SIZE as u32).unwrap();
            self.samples_written = 0;
            self.buffer.clear()
        }
    }
}
