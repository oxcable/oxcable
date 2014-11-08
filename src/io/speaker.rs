//! Provides an output audio stream.

#![experimental]

extern crate portaudio;

use std::vec::Vec;

use core::{SAMPLE_RATE, AudioDevice, Sample, Time};
use core::channel::InputChannelArray;
use io::{BUFFER_SIZE, PORTAUDIO_T};


/// Writes audio to the OS's default output device
pub struct Speaker {
    /// The input array, to receive final audio to write out
    pub inputs: InputChannelArray,

    pa_stream: portaudio::pa::PaStream<Sample>,
    num_channels: uint, 
    buffer: Vec<Sample>,
    samples_written: uint,
}

impl Speaker {
    /// Opens a portaudio stream writing `num_channels` outputs
    pub fn new(num_channels: uint) -> Speaker {
        // Initialize portaudio
        if portaudio::pa::initialize() != portaudio::types::PaNoError {
            panic!("failed to initialize portaudio");
        }

        // Open a stream
        let mut pa_stream = portaudio::pa::PaStream::new(PORTAUDIO_T);
        pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32, 0i32,
                               num_channels as i32, PORTAUDIO_T);
        pa_stream.start();

        Speaker {
            inputs: InputChannelArray::new(num_channels),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_written: 0,
        }
    }

    #[experimental="this should be replaced with a destructor"]
    /// Closes the portaudio stream
    pub fn stop(&mut self) { 
        self.pa_stream.stop();
        self.pa_stream.close();
        portaudio::pa::terminate();
    }
}

impl AudioDevice for Speaker {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let mut s = self.inputs.get_sample(i, t).unwrap_or(0.0);
            if s > 1.0 { s = 1.0; }
            if s < -1.0 { s = -1.0; }
            self.buffer.push(s)
        }
        self.samples_written += 1;

        if self.samples_written == BUFFER_SIZE {
            self.pa_stream.write(self.buffer.clone(), BUFFER_SIZE as u32);
            self.samples_written = 0;
            self.buffer.clear()
        }
    }
}
