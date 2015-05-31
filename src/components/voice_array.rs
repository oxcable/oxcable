//! Provides polyphonic voice arrays.

use std::collections::{VecDeque, HashMap};
use std::vec::Vec;

use components::channel::ChannelRef;
use types::{Device, MidiEvent, MidiMessage, Time, Sample};
use mixers::Adder;


/// A voice that can be used in a `VoiceArray`.
pub trait Voice {
    /// Returns the output channel for our voice.
    fn get_channel(&self) -> ChannelRef<Sample>;

    /// Handle the provided MIDI event
    fn handle_event(&mut self, event: &MidiEvent, t: Time);
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
    /// Maps MIDI note numbers to currently triggered voice indices
    note_to_voice: HashMap<u8, usize>,
    /// Maps voice indices to MIDI note numbers
    voice_to_note: HashMap<usize, u8>,
    /// Places the most recently mapped voices at the end
    voice_queue: VecDeque<usize>
}

impl<T: Device+Voice> VoiceArray<T> {
    /// Creates a new VoiceArray from the provifed vector of voices.
    ///
    /// The VoiceArray takes ownership of the provided voices and ticks them.
    pub fn new(voices: Vec<T>) -> VoiceArray<T> {
        let num_voices = voices.len();
        let mut adder = Adder::new(num_voices);
        let mut voice_queue = VecDeque::new();
        for i in (0 .. num_voices) {
            adder.inputs.set_channel(i, voices[i].get_channel());
            voice_queue.push_back(i);
        }

        VoiceArray {
            output: adder,
            voices: voices,
            note_to_voice: HashMap::new(),
            voice_to_note: HashMap::new(),
            voice_queue: voice_queue
        }
    }

    /// Dispatches the provided MIDI event.
    ///
    /// For NoteOn events, we select a voice to handle the event. For NoteOff
    /// events, we find the voice that was handling the NoteOn event. For all
    /// other events, we send the event to every voice.
    pub fn handle_event(&mut self, event: &MidiEvent, t: Time) {
        match event.payload {
            MidiMessage::NoteOn(note,_) =>
                self.handle_note_on(note as u8, event, t),
            MidiMessage::NoteOff(note,_) =>
                self.handle_note_off(note as u8, event, t),
            _ => self.handle_other_event(event, t)
        }
    }

    /// Selects a voice to handle this event, and triggers the note
    fn handle_note_on(&mut self, note: u8, event: &MidiEvent, t: Time) {
        let i = match self.note_to_voice.remove(&note) {
            Some(i) => {
                // This note is already being played, so retrigger it and move
                // it to the back of the queue
                self.remove_from_queue(i);
                i
            },
            None => {
                // This note is not being played, so get the next voice
                let i = self.voice_queue.pop_front().unwrap();

                // Remove the mapping between the voice and its old note
                match self.voice_to_note.remove(&i) {
                    Some(n) => { self.note_to_voice.remove(&n); },
                    None => ()
                }

                // Map this voice to its new note
                self.note_to_voice.insert(note, i);
                self.voice_to_note.insert(i, note);
                i
            }
        };
        // Finally, push the voice to the back of the queue and handle the event
        self.voice_queue.push_back(i);
        self.voices[i].handle_event(event, t);
    }

    // Find the voice playing this note and stop it
    fn handle_note_off(&mut self, note: u8, event: &MidiEvent, t: Time) {
        match self.note_to_voice.remove(&note) {
            Some(i) => {
                self.remove_from_queue(i);
                self.voice_to_note.remove(&i);
                self.voice_queue.push_front(i);
                self.voices[i].handle_event(event, t);
            },
            None => ()
        }
    }

    // Pass any other event to all the voices
    fn handle_other_event(&mut self, event: &MidiEvent, t: Time) {
        for voice in self.voices.iter_mut() {
            voice.handle_event(event, t);
        }
    }

    // Finds a voice in the queue and removes it
    fn remove_from_queue(&mut self, voice: usize) {
        for i in (0 .. self.voice_queue.len()) {
            if *self.voice_queue.get(i).unwrap() == voice {
                self.voice_queue.remove(i);
                break;
            }
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
