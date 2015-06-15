//! Reads in MIDI events and uses them to trigger an ADSR

extern crate oxcable;

use oxcable::adsr::{Adsr, AdsrMessage};
use oxcable::chain::DeviceChain;
use oxcable::types::{AudioDevice, DeviceIOType, MidiDevice, MidiMessage, Time,
                     Sample};
use oxcable::io::audio::AudioEngine;
use oxcable::io::midi::{MidiEngine, MidiIn};
use oxcable::oscillator::{self, Oscillator};
use oxcable::utils::tick::tick_until_enter;

struct WrappedAdsr {
    midi: MidiIn,
    adsr: Adsr
}
impl AudioDevice for WrappedAdsr {
    fn num_inputs(&self) -> DeviceIOType {
        self.adsr.num_inputs()
    }

    fn num_outputs(&self) -> DeviceIOType {
        self.adsr.num_outputs()
    }

    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for event in self.midi.get_events(t) {
            println!("{:?}", event);
            match event.payload {
                MidiMessage::NoteOn(_,_) =>
                    self.adsr.handle_message(AdsrMessage::NoteDown, t),
                MidiMessage::NoteOff(_,_) =>
                    self.adsr.handle_message(AdsrMessage::NoteUp, t),
                _ => ()
            }
        }
        self.adsr.tick(t, inputs, outputs);
    }
}

#[cfg(not(test))]
fn main() {

    println!("Initializing signal chain...");
    let audio_engine = AudioEngine::open().unwrap();
    let midi_engine = MidiEngine::open().unwrap();


    let mut chain = DeviceChain::from(
        Oscillator::new(oscillator::Sine, 440.0)
    ).into(
        WrappedAdsr {
            midi: midi_engine.choose_input(),
            adsr: Adsr::default(1)
        }
    ).into(
        audio_engine.default_output(1)
    );

    println!("Playing. Press Enter to quit...");
    tick_until_enter(&mut chain);
    println!("Done!");
}
