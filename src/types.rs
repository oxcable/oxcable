//! Global types and constants.


/// The global sample rate, in Hz.
pub static SAMPLE_RATE: u32 = 44100;

/// The datatype of a single audio sample.
pub type Sample = f32;

/// The datatype of a single time tick.
pub type Time = u64;


/// The datatype of a MIDI event.
#[derive(Clone, Copy, Debug)]
pub struct MidiEvent {
    /// The MIDI channel this event was sent to.
    pub channel: u8,
    /// The timestamp of this event.
    pub time: Time,
    /// The message contents.
    pub payload: MidiMessage
}

/// The contents of a MIDI message.
///
/// This enum wraps a useful subset of possible MIDI message. For further
/// details on what these message represent, consult a complete MIDI
/// documentation.
///
/// Certain messages are parsed out to more useful datatypes:
///
///  * Velocities are converted to floats between 0.0 and 1.0
///  * Pressures are converted to floats between 0.0 and 1.0
///  * Bend is converted to a float from -1.0 to 1.0
///
/// Any MIDI type not specifically enumerated preserves the raw bytes as `Other`.
#[derive(Clone, Copy, Debug)]
pub enum MidiMessage {
    /// NoteOn(note number, velocity)
    NoteOn(u8, f32),
    /// NoteOff(note number, velocity)
    NoteOff(u8, f32),
    /// PitchBend(bend)
    PitchBend(f32),
    /// PolyphonicAftertouch(note number, pressure)
    PolyphonicAftertouch(u8, f32),
    /// ChannelAftertouch(pressure)
    ChannelAftertouch(f32),
    /// SustainPedal(on/off)
    SustainPedal(bool),
    /// Portamento(on/off)
    Portamento(bool),
    /// Sostenuto(on/off)
    Sostenuto(bool),
    /// SoftPedal(on/off)
    SoftPedal(bool),
    /// LegatoPedal(on/off)
    LegatoPedal(bool),
    /// Hold2(on/off)
    Hold2(bool),
    /// ControlChange(controller, value)
    ControlChange(u8, u8),
    /// ProgramChange(num)
    ProgramChange(u8),
    /// Other(status, byte1, byte2)
    Other(u8, u8, u8)
}


/// A device that processes and/or generates audio.
pub trait AudioDevice {
    /// Returns the number of input channels the device accepts.
    fn num_inputs(&self) -> usize;

    /// Return sthe number of output channels the device returns.
    fn num_outputs(&self) -> usize;

    /// Processes a single frame worth of audio data. This function should be
    /// called once per time step, starting at `t=0`.
    ///
    /// If a device accepts no inputs, or generates no outputs, then zero length
    /// slices may be passed in.
    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]);
}


/// A device that receives messages.
pub trait MessageReceiver {
    /// The type of messages to be received.
    type Msg;

    /// Handles the message.
    fn handle_message(&mut self, msg: Self::Msg);
}


/// A device that generates MIDI events.
pub trait MidiDevice {
    /// Returns any events scheduled for time `t`.
    fn get_events(&mut self, t: Time) -> Vec<MidiEvent>;
}
