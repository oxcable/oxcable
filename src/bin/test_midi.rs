//! Test script for monitoring MIDI input.
//!
//! The test operates on two levels:
//!
//! 1. All MIDI events are printed to the console.
//! 2. NoteOn/NoteOff events are used to gate the signal.

extern crate oxcable;


#[cfg(not(test))]
mod gate {
    use oxcable::io::midi::MidiIn;
    use oxcable::types::{AudioDevice, MidiDevice, MidiMessage, Time, Sample};

    pub struct Gate {
        pub midi: MidiIn,
        pub enabled: bool,
    }
    impl Gate {
        pub fn new(midi: MidiIn) -> Self {
            Gate {
                midi: midi,
                enabled: false
            }
        }
    }
    impl AudioDevice for Gate {
        fn num_inputs(&self) -> usize { 1 }
        fn num_outputs(&self) -> usize { 1 }
        fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
            for event in self.midi.get_events(t) {
                println!("{:?}", event);
                match event.payload {
                    MidiMessage::NoteOn(_,_) => self.enabled = true,
                    MidiMessage::NoteOff(_,_) => self.enabled = false,
                    _ => ()
                }
            }
            outputs[0] = if self.enabled { inputs[0] } else { 0.0 };
        }
    }
}


#[cfg(not(test))]
fn main() {
    use oxcable::chain::{DeviceChain, Tick};
    use oxcable::io::audio::AudioEngine;
    use oxcable::io::midi::MidiEngine;
    use oxcable::oscillator::{self, Oscillator};

    println!("Initializing signal chain...");
    let audio_engine = AudioEngine::with_buffer_size(256).unwrap();
    let midi_engine = MidiEngine::open().unwrap();

    let mut chain = DeviceChain::from(
        Oscillator::new(oscillator::Saw(oscillator::PolyBlep)).freq(220.0)
    ).into(
        gate::Gate::new(midi_engine.choose_input().unwrap())
    ).into(
        audio_engine.default_output(1).unwrap()
    );

    println!("Playing. Press Enter to quit...");
    chain.tick_until_enter();
    println!("Done!");
}
