//! Provides an input audio stream.

#![experimental]

extern crate portaudio;

use core::types::{SAMPLE_RATE, Device, Sample, Time};
use core::components::OutputArray;
use core::init;
use io::{BUFFER_SIZE, PORTAUDIO_T};


/// Reads audio from the OS's default input device
pub struct Microphone {
    /// The output array holding read audio
    pub outputs: OutputArray<Sample>,

    pa_stream: portaudio::pa::Stream<Sample>,
    num_channels: uint, 
    buffer: Vec<Sample>,
    samples_read: uint,
}

impl Microphone {
    /// Opens a portaudio stream reading `num_channels` inputs
    pub fn new(num_channels: uint) -> Microphone {
        // Check for initialization
        if !init::is_initialized() {
            panic!("Must initialize oxcable first");
        }
        
        // Open a stream
        let mut pa_stream = portaudio::pa::Stream::new(PORTAUDIO_T);
        assert!(pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                                       num_channels as i32, 0i32,
                                       PORTAUDIO_T).is_ok());
        assert!(pa_stream.start().is_ok());

        Microphone {
            outputs: OutputArray::new(num_channels),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_read: BUFFER_SIZE,
        }
    }

    /// Closes the portaudio stream
    pub fn stop(&mut self) {
        assert!(self.pa_stream.stop().is_ok());
        assert!(self.pa_stream.close().is_ok());
    }
}

impl Device for Microphone {
    fn tick(&mut self, _t: Time) {
        if self.samples_read == BUFFER_SIZE {
            let result = self.pa_stream.read(BUFFER_SIZE as u32);
            match result {
                Ok(v) => self.buffer.clone_from(&v),
                Err(e) => panic!(e)
            }
            self.samples_read = 0;
        }

        for i in range(0, self.num_channels) {
            let s = self.buffer[self.samples_read*self.num_channels + i];
            self.outputs.push(i, s);
        }
        self.samples_read += 1;
    }
}
