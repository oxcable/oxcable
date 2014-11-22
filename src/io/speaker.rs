//! Provides an output audio stream.

#![experimental]

extern crate portaudio;

use std::vec::Vec;

use core::types::{SAMPLE_RATE, Device, Sample, Time};
use core::components::InputArray;
use core::init;
use io::{BUFFER_SIZE, PORTAUDIO_T};


/// Writes audio to the OS's default output device
pub struct Speaker {
    /// The input array, to receive final audio to write out
    pub inputs: InputArray<Sample>,

    pa_stream: portaudio::pa::Stream<Sample>,
    num_channels: uint, 
    buffer: Vec<Sample>,
    samples_written: uint,
}

impl Speaker {
    /// Opens a portaudio stream writing `num_channels` outputs
    pub fn new(num_channels: uint) -> Speaker {
        // Check for initialization
        if !init::is_initialized() {
            panic!("Must initialize oxcable first");
        }

        // Open a stream
        let mut pa_stream = portaudio::pa::Stream::new(PORTAUDIO_T);
        assert!(pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                                       0i32, num_channels as i32,
                                       PORTAUDIO_T).is_ok());
        assert!(pa_stream.start().is_ok());

        Speaker {
            inputs: InputArray::new(num_channels),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_written: 0,
        }
    }

    /// Closes the portaudio stream
    pub fn stop(&mut self) {
        assert!(self.pa_stream.stop().is_ok());
        assert!(self.pa_stream.close().is_ok());
    }
}

impl Device for Speaker {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let mut s = self.inputs.get(i, t).unwrap_or(0.0);
            if s > 1.0 { s = 1.0; }
            if s < -1.0 { s = -1.0; }
            self.buffer.push(s)
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
