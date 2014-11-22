//! Provides single channel time synchronized input and output.

#![experimental]

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use core::components::channel::{Channel, ChannelRef};
use core::types::Time;


/// Stores generated frame data and manages access to that data.
#[experimental]
pub struct OutputElement<T> {
    ch: ChannelRef<T>
}

impl<T: Clone+Default> OutputElement<T> {
    /// Creates a new output element, initialized to time `t=0`.
    pub fn new() -> OutputElement<T> {
        OutputElement { ch: Rc::new(RefCell::new(Channel::new())) }
    }

    /// Returns a reference to the channel.
    pub fn get_channel(&self) -> ChannelRef<T> {
        self.ch.clone()
    }

    /// Attempts to get the data frame for time `t`.
    pub fn get(&self, t: Time) -> Option<T> {
        self.ch.borrow_mut().get(t)
    }

    /// Pushes the next data frame.
    pub fn push(&self, f: T) {
        self.ch.borrow_mut().push(f);
    }
}


/// Holds references to channels to draw input data frames from.
#[experimental]
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
    pub fn get(&self, t: Time) -> Option<T> {
        match self.ch {
            Some(ref ch) => ch.borrow().get(t),
            None => None
        }
    }
}

