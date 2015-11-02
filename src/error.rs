//! Provides a standardized error type for oxcable.

use std::io;
use std::result;

use byteorder;
use portmidi;

#[derive(Debug)]
pub enum Error {
    /// No audio resources can be opened.
    NoAudioDevices,
    /// No MIDI resources can be opened.
    NoMidiDevices,
    /// A file is not formatted properly.
    InvalidFile,
    /// A requested index is out of range.
    ///
    /// The string argument specifies which index was out of range.
    OutOfRange(&'static str),
    /// The requested operation would create a graph cycle.
    CreatesCycle,
    /// A std::io operation failed.
    Io(io::Error),
    /// A portmidi operation failed.
    PortMidi(portmidi::PortMidiError),
    /// A feature isn't supported.
    ///
    /// The string argument describes what feature.
    Unsupported(&'static str),
}

impl From<byteorder::Error> for Error {
    fn from(e: byteorder::Error) -> Self {
        match e {
            byteorder::Error::UnexpectedEOF => Error::InvalidFile,
            byteorder::Error::Io(e) => Error::Io(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<portmidi::PortMidiError> for Error {
    fn from(e: portmidi::PortMidiError) -> Error {
        Error::PortMidi(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
