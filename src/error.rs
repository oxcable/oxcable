//! Provides a standardized error type for oxcable.

use std::fmt;
use std::io;
use std::result;

use byteorder;
use portaudio::pa;
use portmidi;

/// A global error type for all oxcable operations.
///
/// Many of the errors simply wrap the errors provided by our supporting
/// libraries.
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
    /// A feature isn't supported.
    ///
    /// The string argument describes what feature.
    Unsupported(&'static str),
    /// A std::io operation failed.
    Io(io::Error),
    /// A portaudio operation failed.
    PortAudio(pa::Error),
    /// A portmidi operation failed.
    PortMidi(portmidi::PortMidiError),
}
use self::Error::*;

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

impl From<pa::Error> for Error {
    fn from(e: pa::Error) -> Error {
        Error::PortAudio(e)
    }
}

impl From<portmidi::PortMidiError> for Error {
    fn from(e: portmidi::PortMidiError) -> Error {
        Error::PortMidi(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &NoAudioDevices => write!(f, "No audio devices found."),
            &NoMidiDevices => write!(f, "No midi devices found."),
            &InvalidFile => write!(f, "The provided was not valid."),
            &OutOfRange(ref idx) => write!(f, "Index `{}` was out of range.", idx),
            &CreatesCycle =>
                write!(f, "The requested action creates a graph cycle."),
            &Unsupported(ref feat) =>
                write!(f, "Unsupported feature: {}", feat),
            &Io(ref e) => write!(f, "IO error: {}", e),
            &PortAudio(ref e) => write!(f, "PortAudio error: {}", e),
            &PortMidi(ref e) => write!(f, "PortMidi error: {:?}", e),
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
