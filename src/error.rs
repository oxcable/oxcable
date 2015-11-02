//! Provides a standardized error type for oxcable.

use std::error;
use std::fmt;
use std::io;
use std::result;

use portmidi;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// Returned when no audio resources can be opened.
    NoAudioDevices,
    /// Returned when no MIDI resources can be opened.
    NoMidiDevices,
    /// Wraps errors returned by portmidi
    PortMidi(portmidi::PortMidiError),
}

impl From<portmidi::PortMidiError> for Error {
    fn from(e: portmidi::PortMidiError) -> Error {
        Error::PortMidi(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
