//! Provides single channel time synchronized input and output.

#![unstable]

use std::default::Default;

use core::components::channel::ChannelRef;
use core::types::Time;


/// Stores generated frame data and manages access to that data.
pub struct OutputElement<T> {
    ch: ChannelRef<T>
}

impl<T: Clone+Default> OutputElement<T> {
    /// Creates a new output element, initialized to time `t=0`.
    pub fn new() -> OutputElement<T> {
        OutputElement { ch: ChannelRef::new() }
    }

    /// Returns a reference to the channel.
    pub fn get_channel(&self) -> ChannelRef<T> {
        self.ch.clone()
    }

    /// Attempts to get the data frame for time `t`.
    #[inline]
    pub fn get(&self, t: Time) -> Option<T> {
        self.ch.get(t)
    }

    /// Pushes the next data frame.
    #[inline]
    pub fn push(&self, f: T) {
        self.ch.push(f);
    }
}


/// Holds references to channels to draw input data frames from.
pub struct InputElement<T> {
    ch: Option<ChannelRef<T>>
}

impl<T: Clone+Default> InputElement<T> {
    /// Creates a new input element, with an empty channel.
    pub fn new() -> InputElement<T> {
        InputElement { ch: None }
    }

    /// Sets the channel to read from `channel`.
    pub fn set_channel(&mut self, channel: ChannelRef<T>) {
        self.ch = Some(channel);
    }

    /// Sets the channel to empty.
    pub fn remove_channel(&mut self) {
        self.ch = None;
    }

    /// Attempts to return the value of the channel at time `t`.
    ///
    /// Returns `None` if either the channel is empty, or if the channel could
    /// not return the sample for time `t`.
    #[inline]
    pub fn get(&self, t: Time) -> Option<T> {
        match self.ch {
            Some(ref ch) => ch.get(t),
            None => None
        }
    }
}

