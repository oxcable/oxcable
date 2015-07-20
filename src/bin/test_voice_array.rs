//! Test script with a simple instrument using `VoiceArray`.
//!
//! The test creates an instrument that can play up to five sine wavs at
//! a time. Then, in the main loop, we check for MIDI events and tick the
//! instrument, sendingn its output to a speaker.

extern crate oxcable;

#[cfg(not(test))]
mod instrument {
    use oxcable::io::midi::MidiIn;
    use oxcable::oscillator::*;
    use oxcable::types::{AudioDevice, MidiDevice, MidiEvent, MidiMessage, Sample, Time};
    use oxcable::voice_array::VoiceArray;
    use oxcable::utils::helpers::midi_note_to_freq;

    struct MyVoice {
        osc: Oscillator,
        playing: bool
    }

    pub struct MyInstrument {
        voices: VoiceArray<MyVoice>,
        midi: MidiIn
    }

    impl MyInstrument {
        pub fn new(midi: MidiIn) -> MyInstrument {
            // Initialize five voices
            let mut voices = Vec::new();
            for _ in 0..5 {
                voices.push(MyVoice {
                    osc: Oscillator::new(Sine),
                    playing: false,
                });
            }

            // Move our voices into the VoiceArray
            MyInstrument {
                voices: VoiceArray::new(voices),
                midi: midi,
            }
        }

        fn handle_event(&mut self, event: MidiEvent) {
            match event.payload {
                MidiMessage::NoteOn(note, _) => {
                    // For a note on, get a new voice to trigger
                    let voice = self.voices.note_on(note);

                    // Mark the voice as playing
                    voice.playing = true;

                    // Set the oscillator frequency to the new note
                    voice.osc.handle_message(SetFreq(midi_note_to_freq(note)));
                },
                MidiMessage::NoteOff(note, _) => {
                    // Find the voice playing this note...
                    let voice = self.voices.note_off(note);

                    // If we found one, stop it from playing
                    voice.map_or((), |d| {
                        d.playing = true;
                    });
                },
                _ => ()
            }
        }
    }

    impl AudioDevice for MyInstrument {
        fn num_inputs(&self) -> usize { 0 }
        fn num_outputs(&self) -> usize { 1 }
        fn tick(&mut self, t: Time, _: &[Sample], outputs: &mut[Sample]) {
            // First handle any events
            for event in self.midi.get_events(t) {
                self.handle_event(event);
            }

            // For each voice in the array...
            outputs[0] = 0.0;
            for voice in self.voices.iter_mut() {
                // Tick the oscillator
                let input = [];
                let mut output = [0.0];
                voice.osc.tick(t, &input, &mut output);

                // If this oscillator is playing, then add it to the output
                // signal
                if voice.playing {
                    outputs[0] += output[0];
                }
            }
        }
    }
}

#[cfg(not(test))]
fn main() {
    use oxcable::chain::{DeviceChain, Tick};
    use oxcable::io::audio::AudioEngine;
    use oxcable::io::midi::MidiEngine;
    use self::instrument::MyInstrument;

    println!("Initializing signal chain...");
    let audio_engine = AudioEngine::with_buffer_size(256).unwrap();
    let midi_engine = MidiEngine::open().unwrap();
    let mut chain = DeviceChain::from(
        MyInstrument::new(midi_engine.choose_input().unwrap())
    ).into(
        audio_engine.default_output(1).unwrap()
    );

    println!("Playing. Press Enter to quit...");
    chain.tick_until_enter();
    println!("Done!");
}
