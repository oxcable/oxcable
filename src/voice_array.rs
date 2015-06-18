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
    /// Places the most recently mapped voices at the end, and track the note
    /// they are currently playing
    held_voices: VecDeque<(usize, u8)>,
    /// Tracks free voices
    free_voices: VecDeque<usize>,
}

impl<T> VoiceArray<T> {
    /// Creates a new VoiceArray from the provifed vector of voices.
    pub fn new(voices: Vec<T>) -> VoiceArray<T> {
        let num_voices = voices.len();
        let mut free_voices = VecDeque::new();
        for i in (0 .. num_voices) {
            free_voices.push_back(i);
        }

        VoiceArray {
            voices: voices,
            note_to_voice: HashMap::new(),
            held_voices: VecDeque::new(),
            free_voices: free_voices,
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
        let i = match self.note_to_voice.get(&note) {
            Some(&i) => {
                // This note is already being played, so retrigger it and move
                // it to the back of the queue
                self.remove_from_queue(i);
                i
            },
            None => {
                let i = match self.free_voices.pop_front() {
                    // If there is a free voice, use the oldest one
                    Some(i) => i,
                    // Otherwise, use the oldest playing voice
                    None => {
                        let (i, n) = self.held_voices.pop_front().unwrap();
                        self.note_to_voice.remove(&n);
                        i
                    }
                };
                self.note_to_voice.insert(note, i);
                i
            }
        };
        // Finally, push the voice to the back of the queue and handle the event
        self.held_voices.push_back((i,note));
        &mut self.voices[i]
    }

    /// Find the voice playing this note and mark it as done
    pub fn note_off(&mut self, note: u8) -> Option<&mut T> {
        match self.note_to_voice.remove(&note) {
            Some(i) => {
                self.remove_from_queue(i);
                self.free_voices.push_back(i);
                Some(&mut self.voices[i])
            },
            None => None
        }
    }

    // Finds a voice in the queue and removes it
    fn remove_from_queue(&mut self, voice: usize) {
        for i in (0 .. self.held_voices.len()) {
            let (j, _) = self.held_voices[i];
            if j == voice {
                self.held_voices.remove(i);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    /// Verify that asking for a new note selects free voices first
    #[test]
    fn test_free_voice() {
        use super::VoiceArray;
        let mut voices1 = VoiceArray::new(vec![1,2]);
        let v1 = voices1.note_on(1).clone();
        let _ = voices1.note_on(2).clone();
        voices1.note_off(1);
        let v3 = voices1.note_on(3).clone();
        assert_eq!(v1, v3);

        let mut voices2 = VoiceArray::new(vec![1,2]);
        let _ = voices2.note_on(1).clone();
        let v2 = voices2.note_on(2).clone();
        voices2.note_off(2);
        let v3 = voices2.note_on(3).clone();
        assert_eq!(v2, v3);
    }

    /// Verify that asking for a held note always returns that held note
    #[test]
    fn test_key_repeat() {
        use super::VoiceArray;
        let mut voices = VoiceArray::new(vec![1,2]);
        let v1 = voices.note_on(1).clone();
        let v2 = voices.note_on(2).clone();
        let v3 = voices.note_on(1).clone();
        let v4 = voices.note_on(1).clone();
        let v5 = voices.note_on(2).clone();
        let v6 = voices.note_on(2).clone();
        assert!(v1 != v2);
        assert_eq!(v1, v3);
        assert_eq!(v1, v4);
        assert_eq!(v2, v5);
        assert_eq!(v2, v6);
    }

    /// Verify that the oldest free voice is always selected
    #[test]
    fn test_oldest_free() {
        use super::VoiceArray;
        let mut voices = VoiceArray::new(vec![1,2]);
        let v1 = voices.note_on(1).clone();
        let v2 = voices.note_on(2).clone();

        voices.note_off(1);
        voices.note_off(2);
        let v3 = voices.note_on(3).clone();
        assert_eq!(v3, v1);

        voices.note_off(3);
        let v4 = voices.note_on(4).clone();
        assert_eq!(v4, v2);
    }

    /// Verify that note pruning always selects the oldest held voice
    #[test]
    fn test_oldest_held() {
        use super::VoiceArray;
        let mut voices = VoiceArray::new(vec![1,2,3]);
        let v1 = voices.note_on(1).clone();
        let v2 = voices.note_on(2).clone();
        let v3 = voices.note_on(3).clone();
        let v4 = voices.note_on(4).clone();
        let v5 = voices.note_on(5).clone();
        let v6 = voices.note_on(6).clone();
        assert_eq!(v1, v4);
        assert_eq!(v2, v5);
        assert_eq!(v3, v6);
    }
}
