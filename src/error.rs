//! Provides a standardized error type for oxcable.

use std::error;
use std::fmt;
use std::result;

use byteorder;

#[doc(no_inline)] pub use portaudio::pa::Error as PortAudioError;
#[doc(no_inline)] pub use portmidi::PortMidiError;
#[doc(no_inline)] pub use std::io::Error as IoError;


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
    Io(IoError),
    /// A portaudio operation failed.
    PortAudio(PortAudioError),
    /// A portmidi operation failed.
    PortMidi(PortMidiError),
}
pub use self::Error::*;

impl From<byteorder::Error> for Error {
    fn from(e: byteorder::Error) -> Self {
        match e {
            byteorder::Error::UnexpectedEOF => Error::InvalidFile,
            byteorder::Error::Io(e) => Error::Io(e),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<PortAudioError> for Error {
    fn from(e: PortAudioError) -> Self {
        Error::PortAudio(e)
    }
}

impl From<PortMidiError> for Error {
    fn from(e: PortMidiError) -> Self {
        Error::PortMidi(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &NoAudioDevices => "No audio devices found",
            &NoMidiDevices => "No midi devices found",
            &InvalidFile => "The provided was not valid",
            &OutOfRange(_) => "Index was out of range",
            &CreatesCycle => "The requested action creates a graph cycle",
            &Unsupported(_) => "Unsupported feature",
            &Io(_) => "std::io error",
            &PortAudio(_) => "PortAudio error",
            &PortMidi(_) => "PortMidi error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Io(ref e) => Some(e),
            &PortAudio(ref e) => Some(e),
            &PortMidi(ref e) => Some(e),
            _ => None,
        }
    }
}

/// A specialized Result for oxcable.
pub type Result<T> = result::Result<T, Error>;
