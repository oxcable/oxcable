//! Provides audio IO from OS sound devices.

extern crate portaudio;
use std::rc::Rc;

use self::portaudio::pa;

use types::{SAMPLE_RATE, AudioDevice, Sample, Time};


/// Defines the audio format for Portaudio.
static PORTAUDIO_T: pa::SampleFormat = pa::SampleFormat::Float32;


/// The AudioEnginer opens and manages the resources associated with portaudio.
/// It is used open new input/output streams and safely free them.
pub struct AudioEngine {
    marker: Rc<AudioEngineMarker>,
    buffer_size: usize,
}

impl AudioEngine {
    pub fn with_buffer_size(samples: usize) -> Result<AudioEngine, pa::Error> {
        try!(portaudio::pa::initialize());
        Ok(AudioEngine {
            marker: Rc::new(AudioEngineMarker),
            buffer_size: samples
        })
    }

    pub fn default_input(&self, num_channels: usize)
            -> Result<AudioIn, pa::Error> {
        AudioIn::new(self, num_channels)
    }

    pub fn default_output(&self, num_channels: usize)
            -> Result<AudioOut, pa::Error> {
        AudioOut::new(self, num_channels)
    }
}


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


/// Reads audio from the OS's default input device.
pub struct AudioIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<AudioEngineMarker>,
    pa_stream: portaudio::pa::Stream<Sample, Sample>,
    num_channels: usize,
    buffer: Vec<Sample>,
    buffer_size: usize,
    samples_read: usize,
}

impl AudioIn {
    /// Opens an audio input stream reading `num_channels` inputs.
    fn new(engine: &AudioEngine, num_channels: usize)
            -> Result<AudioIn, pa::Error> {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        try!(pa_stream.open_default(SAMPLE_RATE as f64,
                                    engine.buffer_size as u32,
                                    num_channels as i32,
                                    0i32,
                                    PORTAUDIO_T,
                                    None));
        try!(pa_stream.start());

        let buf_size = num_channels*engine.buffer_size;
        Ok(AudioIn {
            engine: engine.marker.clone(),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: vec![0.0; buf_size],
            samples_read: engine.buffer_size,
            buffer_size: engine.buffer_size,
        })
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
        if self.samples_read == self.buffer_size {
            let num_read = self.num_channels * self.buffer_size;
            match self.pa_stream.read(num_read as u32) {
                Ok(v) => {
                    for (i, &s) in v.iter().enumerate() {
                        self.buffer[i] = s;
                    }
                },
                Err(portaudio::pa::Error::InputOverflowed) => {
                    println!("Input overflowed");
                    for i in 0..self.buffer.len() {
                        self.buffer[i] = 0.0;
                    }
                },
                Err(e) => panic!("{}", e)
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
    buffer_size: usize,
    samples_written: usize,
}

impl AudioOut {
    /// Opens an output stream writing `num_channels` outputs.
    fn new(engine: &AudioEngine, num_channels: usize)
            -> Result<AudioOut, pa::Error> {
        // Open a stream in blocking mode
        let mut pa_stream = portaudio::pa::Stream::new();
        try!(pa_stream.open_default(SAMPLE_RATE as f64,
                                    engine.buffer_size as u32,
                                    0i32,
                                    num_channels as i32,
                                    PORTAUDIO_T,
                                    None));
        try!(pa_stream.start());
        Ok(AudioOut {
            engine: engine.marker.clone(),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*engine.buffer_size),
            buffer_size: engine.buffer_size,
            samples_written: 0,
        })
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

        if self.samples_written == self.buffer_size {
            match self.pa_stream.write(self.buffer.clone(),
                                 self.buffer_size as u32) {
                Ok(()) => (),
                Err(pa::Error::OutputUnderflowed) => println!("Output underflowed"),
                Err(e) => panic!("{}", e)
            }
            self.samples_written = 0;
            self.buffer.clear()
        }
    }
}
