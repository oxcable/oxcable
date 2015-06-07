//! Provides polyphonic voice arrays.

use std::collections::{VecDeque, HashMap};
use std::slice::{Iter, IterMut};
use std::vec::Vec;


/// A manager for a polyphonic set of voices.
pub struct VoiceArray<T> {
    /// All the voices are contained in the voices vector
    voices: Vec<T>,
    /// Maps MIDI note numbers to currently triggered voice indices
    note_to_voice: HashMap<u8, usize>,
    /// Maps voice indices to MIDI note numbers
    voice_to_note: HashMap<usize, u8>,
    /// Places the most recently mapped voices at the end
    voice_queue: VecDeque<usize>
}

impl<T> VoiceArray<T> {
    /// Creates a new VoiceArray from the provifed vector of voices.
    pub fn new(voices: Vec<T>) -> VoiceArray<T> {
        let num_voices = voices.len();
        let mut voice_queue = VecDeque::new();
        for i in (0 .. num_voices) {
            voice_queue.push_back(i);
        }

        VoiceArray {
            voices: voices,
            note_to_voice: HashMap::new(),
            voice_to_note: HashMap::new(),
            voice_queue: voice_queue
        }
    }

    /// Get an iterator over the voice objects
    pub fn iter(&self) -> Iter<T> {
        self.voices.iter()
    }

    /// Get a mutable iterator over the voice objects
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.voices.iter_mut()
    }

    /// Selects a new voice, marks it as used, and loans it out
    pub fn note_on(&mut self, note: u8) -> &mut T {
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
        &mut self.voices[i]
    }

    // Find the voice playing this note and mark it as done
    pub fn note_off(&mut self, note: u8) -> Option<&mut T> {
        match self.note_to_voice.remove(&note) {
            Some(i) => {
                self.remove_from_queue(i);
                self.voice_to_note.remove(&i);
                self.voice_queue.push_front(i);
                Some(&mut self.voices[i])
            },
            None => None
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
