//! Time synchronized input and output components for processing devices.
//!
//! These components are meant to provide standard building blocks and interface
//! elements to create `Device`s.
//! 
//! Input/Output Componenets
//! ------------------------
//!
//! A device that wants to receive data can use an `InputElement` if it only
//! wants a single channel, or an `InputArray` if it desires multiple channels
//! of input. A device that wants to create data can use an `OutputElement` if
//! it only wants a single channel, or an `OutputArray` if it desires multiple
//! channels of output.
//!
//! For example, consider a simple synthesizer. This synthesizer may want to
//! receive notes from a MIDI keyboard, and generate stereo audio. In this case,
//! we would want to give it a public `InputElement<MidiEvent>` to receive
//! a single set of MIDI events, and a public `OutputArray<Sample>` with two
//! channels, one for each stereo audio channel.
//!
//! Messages
//! --------
//!
//! A device that wishes to receive arbitrary messages from the outside world
//! can use a `MessageReceiver`, which can then create multiple thread-safe
//! `MessageSender`s to send messages with.

#![unstable]

pub use self::io_array::{InputArray, OutputArray};
pub use self::io_elements::{InputElement, OutputElement};
pub use self::voice_array::{Voice, VoiceArray};

pub mod channel;
pub mod io_array;
pub mod io_elements;
pub mod voice_array;
