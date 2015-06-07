//! Defines types and constants to be used globally in oxcable.


/// The global sample rate, in Hz.
pub static SAMPLE_RATE: u32 = 44100;

/// The datatype of a single sample.
pub type Sample = f32;

/// The datatype of a single sample time.
pub type Time   = u64;


/// The datatype of a midi event.
#[derive(Clone, Copy, Debug)]
pub struct MidiEvent {
    /// The MIDI channel this event was sent to
    pub channel: u8,
    /// The timestamp of this event
    pub time: Time,
    /// The message contents
    pub payload: MidiMessage
}

/// The contents of a MIDI Message
///
/// Certain messages are parsed out to more useful datatypes:
///
///  * Velocities are converted to floats between 0.0 and 1.0
///  * Pressures are converted to floats between 0.0 and 1.0
///  * Bend is converted to a float from -1.0 to 1.0
#[derive(Clone, Copy, Debug)]
pub enum MidiMessage {
    /// NoteOn(note number, velocity)
    NoteOn(u8, f32),
    /// NoteOff(note number, velocity)
    NoteOff(u8, f32),
    /// PitchBend(bend)
    PitchBend(f32),
    /// KeyPressure(note number, pressure)
    KeyPressure(u8, f32),
    /// ControlChange(controller, value)
    ControlChange(u8, u8),
    /// ProgramChange(num)
    ProgramChange(u8),
    /// ChannelPressure(pressure)
    ChannelPressure(f32),
    /// Other(status, byte1, byte2)
    Other(u8, u8, u8)
}



#[derive(Clone, Copy, Debug)]
pub enum DeviceIOType {
    Any,
    Exactly(usize)
}

pub trait AudioDevice {
    fn num_inputs(&self) -> DeviceIOType;
    fn num_outputs(&self) -> DeviceIOType;

    /// Process a single frame worth of data. This function should be called
    /// once per time step, starting at `t=0`.
    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]);
}
