//! Time synchronous container for data

#![experimental]

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use core::types::Time;


/// A reference to a channel, used to link outputs to inputs.
pub type ChannelRef<T> = Rc<RefCell<Channel<T>>>;

/// Container for a single channel of data.
#[experimental]
pub struct Channel<T> {
    next_t: Time,
    data: T, 
}

impl<T: Clone+Default> Channel<T> {
    /// Returns a new channel, initialized to time 0
    pub fn new() -> Channel<T> {
        Channel{ next_t: 0, data: Default::default() }
    }

    /// Attempts to return the data for time `t`.
    ///
    /// Returns `None` if we don't have the requested time.
    pub fn get(&self, t: Time) -> Option<T> {
        if t != self.next_t-1 {
            None
        } else {
            Some(self.data.clone())
        }
    }

    /// Add the next sample to the channel
    pub fn push(&mut self, s: T) {
        self.data = s.clone();
        self.next_t += 1;
    }
}
