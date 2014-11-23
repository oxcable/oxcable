//! Provides polyphonic voice arrays.

#![experimental]

use std::collections::{HashMap, RingBuf};
use std::vec::Vec;

use core::components::channel::ChannelRef;
use core::types::{Device, MidiEvent, MidiMessage, Time, Sample};
use mixers::Adder;


/// A voice that can be used in a `VoiceArray`.
pub trait Voice {
    /// Returns the output channel for our voice.
    fn get_channel(&self) -> ChannelRef<Sample>;

    /// Handle the provided MIDI event
    fn handle_event(&mut self, event: MidiEvent);
}


/// A manager for a polyphonic set of voices.
///
/// A VoiceArray takes in set of Voice objects, and manages them automatically.
/// When a MIDI event comes in, the array intelligently handles it. Note ons
/// will trigger only one voice, note offs will stop the voice handling that
/// note, and other messages will be passed to every note.
pub struct VoiceArray<T> {
    /// All voices have their output fed to the adder
    pub output: Adder,

    /// All the voices are contained in the voices vector
    voices: Vec<T>,
    /// The voice_queue contains the indices of all voices, with free voices in
    /// front, and the most recently triggered notes in the back.
    voice_queue: RingBuf<uint>,
    /// The note_to_voice maps MIDI note numbers to currently triggered voice
    /// indices.
    note_to_voice: HashMap<u8, uint>
}

impl<T: Device+Voice> VoiceArray<T> {
    /// Creates a new VoiceArray from the provifed vector of voices.
    ///
    /// The VoiceArray takes ownership of the provided voices and ticks them.
    pub fn new(voices: Vec<T>) -> VoiceArray<T> {
        let num_voices = voices.len();
        let mut adder = Adder::new(num_voices);
        let mut voice_queue = RingBuf::with_capacity(num_voices);
        for i in range(0, num_voices) {
            voice_queue.push_back(i);
            adder.inputs.set_channel(i, voices[i].get_channel());
        }

        VoiceArray {
            output: adder,
            voices: voices,
            voice_queue: voice_queue,
            note_to_voice: HashMap::new()
        }
    }

    /// Dispatches the provided MIDI event.
    ///
    /// For NoteOn events, we select a voice to handle the event. For NoteOff
    /// events, we find the voice that was handling the NoteOn event. For all
    /// other events, we send the event to every voice.
    pub fn handle_event(&mut self, event: MidiEvent) {
        match event.payload {
            MidiMessage::NoteOn(note,_) => self.handle_note_on(note, event),
            MidiMessage::NoteOff(note,_) => self.handle_note_off(note, event),
            _ => self.handle_other_event(event)
        }
    }

    /// Select a voice, move it to the back of our queue and trigger it
    fn handle_note_on(&mut self, note: u8, event: MidiEvent) {
        let i = if self.note_to_voice.contains_key(&note) {
            // TODO: move this note to the back of voice_queu
            *self.note_to_voice.get(&note).unwrap()
        } else {
            let i = self.voice_queue.pop_front().unwrap();
            self.voice_queue.push_back(i);
            i
        };
        self.voices[i].handle_event(event);
    }

    fn handle_note_off(&mut self, note: u8, event: MidiEvent) {
        match self.note_to_voice.remove(&note) {
            Some(i) => self.voices[i].handle_event(event),
            None => ()
        }
    }

    fn handle_other_event(&mut self, event: MidiEvent) {
        for voice in self.voices.iter_mut() {
            voice.handle_event(event);
        }
    }
}

impl<T: Device+Voice> Device for VoiceArray<T> {
    fn tick(&mut self, t: Time) {
        for voice in self.voices.iter_mut() {
            voice.tick(t);
        }
        self.output.tick(t);
    }
}
