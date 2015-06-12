//! A basic subtractive synthesizer

use adsr::{Adsr, AdsrMessage};
use io::midi::MidiIn;
use oscillator::{AntialiasType, Oscillator, OscillatorMessage, Waveform};
use types::{AudioDevice, DeviceIOType, MidiEvent, MidiMessage, Time, Sample};
use utils::helpers::midi_note_to_freq;
use instruments::voice_array::VoiceArray;


/// A polyphonic subtractive synthesizer
pub struct SubtractiveSynth {
    voices: VoiceArray<SubtractiveSynthVoice>,
    midi: MidiIn,
    gain: f32,
}

impl SubtractiveSynth {
    /// Returns a new subtractive synth that can play `num_voices` notes at one
    /// time.
    pub fn new(midi: MidiIn, num_voices: usize) -> SubtractiveSynth {
        let mut voices = Vec::with_capacity(num_voices);
        for _i in (0 .. num_voices) {
            voices.push(SubtractiveSynthVoice::new());
        }
        let voice_array = VoiceArray::new(voices);

        SubtractiveSynth {
            voices: voice_array,
            midi: midi,
            gain: -12.0,
        }
    }

    fn handle_event(&mut self, event: MidiEvent) {
        match event.payload {
            MidiMessage::NoteOn(note, _) =>
                self.voices.note_on(note).handle_event(event),
                MidiMessage::NoteOff(note, _) =>
                    self.voices.note_off(note).map_or((),
                        |d| d.handle_event(event)),
                _ => {
                    for voice in self.voices.iter_mut() {
                        voice.handle_event(event);
                    }
                }
        }
    }
}

impl AudioDevice for SubtractiveSynth {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(0)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(1)
    }

    fn tick(&mut self, t: Time, _: &[Sample], outputs: &mut[Sample]) {
        for event in self.midi.get_events(t) {
            self.handle_event(event);
        }

        let mut s = 0.0;
        for voice in self.voices.iter_mut() {
            s += voice.tick(t);
        }
        outputs[0] = self.gain * s;
    }
}


/// The container for a single voice
struct SubtractiveSynthVoice {
    osc: Oscillator,
    adsr: Adsr,
    osc_buf: Vec<Sample>,
    adsr_buf: Vec<Sample>,
}

impl SubtractiveSynthVoice {
    fn new() -> SubtractiveSynthVoice {
        let osc = Oscillator::new(Waveform::Saw(AntialiasType::PolyBlep), 440.0);
        let adsr = Adsr::default(1);
        SubtractiveSynthVoice {
            osc: osc,
            adsr: adsr,
            osc_buf: vec![0.0],
            adsr_buf: vec![0.0],
        }
    }

    fn handle_event(&mut self, event: MidiEvent) {
        match event.payload {
            MidiMessage::NoteOn(note, _) => {
                self.osc.handle_message(OscillatorMessage::SetFreq(
                        midi_note_to_freq(note)));
                self.adsr.handle_message(AdsrMessage::NoteDown, event.time);
            },
            MidiMessage::NoteOff(_, _) =>
                self.adsr.handle_message(AdsrMessage::NoteUp, event.time),
            _ => ()
        }
    }

    fn tick(&mut self, t: Time) -> Sample {
        self.osc.tick(t, &[0.0;0], &mut self.osc_buf);
        self.adsr.tick(t, &self.osc_buf, &mut self.adsr_buf);
        self.adsr_buf[0]
    }
}
