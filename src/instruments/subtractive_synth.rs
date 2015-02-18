//! A basic subtractive synthesizer

#![unstable]

use adsr::{Adsr, AdsrMessage};
use components::{InputElement, Voice, VoiceArray};
use components::channel::ChannelRef;
use types::{Device, MidiEvent, MidiMessage, Time, Sample};
use utils::helpers::midi_note_to_freq;
use mixers::Gain;
use oscillator::{AntialiasType, Oscillator, OscillatorMessage, Waveform};


/// A polyphonic subtractive synthesizer
pub struct SubtractiveSynth {
    /// Input MIDI channel
    #[stable]
    pub input: InputElement<Vec<MidiEvent>>,
    /// Output audio channel
    #[stable]
    pub output: Gain,

    /// Our voice manager
    voices: VoiceArray<SubtractiveSynthVoice>
}

impl SubtractiveSynth {
    /// Returns a new subtractive synth that can play `num_voices` notes at one
    /// time.
    #[stable]
    pub fn new(num_voices: usize) -> SubtractiveSynth {
        let mut voices = Vec::with_capacity(num_voices);
        for _i in (0 .. num_voices) {
            voices.push(SubtractiveSynthVoice::new());
        }
        let voice_array = VoiceArray::new(voices);

        let mut output = Gain::new(-6.0, 1);
        output.inputs.set_channel(0, voice_array.output.output.get_channel());

        SubtractiveSynth {
            input: InputElement::new(),
            output: output,
            voices: voice_array
        }
    }
}

impl Device for SubtractiveSynth {
    fn tick(&mut self, t: Time) {
        let events = self.input.get(t).unwrap();
        for event in events.iter() {
            self.voices.handle_event(event, t);
        }

        self.voices.tick(t);
        self.output.tick(t);
    }
}


/// The container for a single voice
struct SubtractiveSynthVoice {
    osc: Oscillator,
    adsr: Adsr
}

impl SubtractiveSynthVoice {
    fn new() -> SubtractiveSynthVoice {
        let osc = Oscillator::new(Waveform::Saw(AntialiasType::PolyBlep), 440.0);
        let mut adsr = Adsr::default(1);
        adsr.inputs.set_channel(0, osc.output.get_channel());
        SubtractiveSynthVoice {
            osc: osc,
            adsr: adsr
        }
    }
}

impl Device for SubtractiveSynthVoice {
    fn tick(&mut self, t: Time) {
        self.osc.tick(t);
        self.adsr.tick(t);
    }
}

impl Voice for SubtractiveSynthVoice {
    fn get_channel(&self) -> ChannelRef<Sample> {
        self.adsr.outputs.get_channel(0)
    }

    fn handle_event(&mut self, event: &MidiEvent, t: Time) {
        match event.payload {
            MidiMessage::NoteOn(note, _) => {
                self.osc.handle_message(OscillatorMessage::SetFreq(
                        midi_note_to_freq(note)));
                self.adsr.handle_message(AdsrMessage::NoteDown, t);
            },
            MidiMessage::NoteOff(_, _) => 
                self.adsr.handle_message(AdsrMessage::NoteUp, t),
            _ => ()
        }
    }
}
