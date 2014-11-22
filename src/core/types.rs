//! Defines types and constants to be used globally in oxcable.


/// The global sample rate, in Hz.
pub static SAMPLE_RATE: uint = 44100;

/// The datatype of a single sample.
pub type Sample = f32;

/// The datatype of a single sample time.
pub type Time   = u64;


/// The datatype of a midi event.
///
/// Status defines the type of message, and the first and second bytes carry
/// various payloads
#[deriving(Clone, Show)]
pub struct MidiEvent {
    pub status: u8,
    pub byte1: u8,
    pub byte2: u8
}

/// Defines names for the different types of MIDI messages.
///
/// Other is used for any unknown message type.
pub enum MidiEventType {
    NoteOn, NoteOff, KeyPressure, ControlChange, ProgramChange, ChannelPressure,
    PitchBend, Other
}

impl MidiEvent {
    /// Parses the event status to return the type of this event.
    pub fn get_type(&self) -> MidiEventType {
        match self.status >> 4 {
            0b1000 => MidiEventType::NoteOff,
            0b1001 => MidiEventType::NoteOn,
            0b1010 => MidiEventType::KeyPressure,
            0b1011 => MidiEventType::ControlChange,
            0b1100 => MidiEventType::ProgramChange,
            0b1101 => MidiEventType::ChannelPressure,
            0b1110 => MidiEventType::PitchBend,
            _ => MidiEventType::Other
        }
    }

    /// Parses the event status to return the MIDI channel for this event.
    pub fn get_channel(&self) -> u8 {
        self.status & 0x0F
    }
}


/// An interface for a synchronous processing device.
#[experimental]
pub trait Device {
    /// Process a single frame worth of data. This function should be called
    /// once per time step, starting at `t=1`.
    fn tick(&mut self, t: Time);
}
