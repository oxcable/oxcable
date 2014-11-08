//! Provides an input audio stream.

#![experimental="should use destructor for closing portaudio"]

extern crate portaudio;

use core::{SAMPLE_RATE, AudioDevice, Sample, Time};
use core::channel::OutputChannelArray;
use io::{BUFFER_SIZE, PORTAUDIO_T};


/// Reads audio from the OS's default input device
pub struct Microphone {
    /// The output array holding read audio
    pub outputs: OutputChannelArray,

    pa_stream: portaudio::pa::PaStream<Sample>,
    num_channels: uint, 
    buffer: Vec<Sample>,
    samples_read: uint,
}

impl Microphone {
    /// Opens a portaudio stream reading `num_channels` inputs
    pub fn new(num_channels: uint) -> Microphone {
        // Initialize portaudio
        if portaudio::pa::initialize() != portaudio::types::PaNoError {
            panic!("failed to initialize portaudio");
        }

        // Open a stream
        let mut pa_stream = portaudio::pa::PaStream::new(PORTAUDIO_T);
        pa_stream.open_default(SAMPLE_RATE as f64, BUFFER_SIZE as u32,
                               num_channels as i32, 0i32, PORTAUDIO_T);
        pa_stream.start();

        Microphone {
            outputs: OutputChannelArray::new(num_channels),
            pa_stream: pa_stream,
            num_channels: num_channels,
            buffer: Vec::with_capacity(num_channels*BUFFER_SIZE),
            samples_read: BUFFER_SIZE,
        }
    }

    /// Closes the portaudio stream
    #[experimental="this should be replaced with a destructor"]
    pub fn stop(&mut self) { 
        self.pa_stream.stop();
        self.pa_stream.close();
        portaudio::pa::terminate();
    }
}

impl AudioDevice for Microphone {
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
            self.outputs.push_sample(i, s);
        }
        self.samples_read += 1;
    }
}
