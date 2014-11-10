//! Time synchronous container for data
//!
//! TODO: Make generic

#![experimental]

use std::cell::RefCell;
use std::rc::Rc;

use core::{Sample, Time};


/// A reference to a channel, used to link outputs to inputs.
pub type ChannelRef = Rc<RefCell<Channel>>;

/// Container for a single channel of data.
#[experimental]
pub struct Channel {
    t: Time,
    data: Sample, 
}

impl Channel {
    /// Returns a new channel, initialized to time 0
    pub fn new() -> Channel {
        Channel{ t: 0, data: 0.0 }
    }

    /// Attempts to return the data for time `t`.
    ///
    /// Returns `None` if we don't have the requested time.
    pub fn get(&self, t: Time) -> Option<Sample> {
        if t != self.t {
            None
        } else {
            Some(self.data)
        }
    }

    /// Add the next sample to the channel
    pub fn push(&mut self, s: Sample) {
        self.t += 1;
        self.data = s;
    }
}
