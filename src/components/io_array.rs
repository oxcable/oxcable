//! Provides time synchronized input and output arrays.

#![unstable]

use std::default::Default;
use std::vec::Vec;

use components::channel::{Channel, ChannelRef};
use types::Time;


/// Stores generated frame data and manages access to that data.
pub struct OutputArray<T> {
    chs: Vec<Channel<T>>
}

impl<T: Clone+Default> OutputArray<T> {
    /// Creates a new output array with `num_channels` channels.
    ///
    /// These channels are initialized to time `t=0`.
    pub fn new(num_channels: usize) -> OutputArray<T> {
        let mut chs = Vec::with_capacity(num_channels);
        for _ in (0 .. num_channels) {
            chs.push(Channel::new());
        }
        OutputArray { chs: chs }
    }

    /// Returns the number of output channels
    pub fn get_num_channels(&self) -> usize {
        self.chs.len()
    }

    /// Returns a reference to channel `i`.
    pub fn get_channel(&self, i: usize) -> ChannelRef<T> {
        self.chs[i].get_reader()
    }

    /// Attempts to get the data frame for time `t` in channel `i`.
    #[inline]
    pub fn get(&self, i: usize, t: Time) -> Option<T> {
        self.chs[i].get(t)
    }

    /// Pushes the next data frame to channel `i`.
    #[inline]
    pub fn push(&mut self, i: usize, f: T) {
        self.chs[i].push(f);
    }
}


/// Holds references to channels to draw input data frames from.
pub struct InputArray<T> {
    chs: Vec<Option<ChannelRef<T>>>
}

impl<T: Clone+Default> InputArray<T> {
    /// Creates a new input array that can receive from `num_channels` channels.
    ///
    /// These channels are initialized as empty, and must be filled to return
    /// input data.
    pub fn new(num_channels: usize) -> InputArray<T> {
        let mut chs = Vec::with_capacity(num_channels);
        for _ in (0 .. num_channels) {
            chs.push(None);
        }
        InputArray { chs: chs }
    }

    /// Returns the number of input channels
    pub fn get_num_channels(&self) -> usize {
        self.chs.len()
    }

    /// Sets channel `i` to read from `channel`.
    pub fn set_channel(&mut self, i: usize, channel: ChannelRef<T>) {
        self.chs[i] = Some(channel);
    }

    /// Sets channel `i` to empty.
    pub fn remove_channel(&mut self, i: usize) {
        self.chs[i] = None;
    }

    /// Attempts to return the value of channel `i` at time `t`.
    ///
    /// Returns `None` if either channel `i` is empty, or if channel `i` could
    /// not return the sample for time `t`.
    #[inline]
    pub fn get(&self, i: usize, t: Time) -> Option<T> {
        match self.chs[i] {
            Some(ref ch) => ch.get(t),
            None => None
        }
    }
}

