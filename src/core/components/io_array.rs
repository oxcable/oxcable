//! Time synchronized input and output arrays

#![experimental]

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use std::vec::Vec;

use core::components::channel::{Channel, ChannelRef};
use core::types::Time;


/// Stores generated audio data and manages access to that data.
#[experimental]
pub struct OutputArray<T> {
    chs: Vec<ChannelRef<T>>
}

impl<T: Clone+Default> OutputArray<T> {
    /// Creates a new output array with `num_channels` channels.
    ///
    /// These channels are initialized at time 0.
    pub fn new(num_channels: uint) -> OutputArray<T> {
        let mut chs = Vec::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            chs.push(Rc::new(RefCell::new(Channel::new())));
        }
        OutputArray { chs: chs }
    }
    
    /// Returns the number of output channels
    pub fn get_num_channels(&self) -> uint {
        self.chs.len()
    }

    /// Returns a reference to channel `i`.
    pub fn get_channel(&self, i: uint) -> ChannelRef<T> {
        self.chs[i].clone()
    }

    /// Attempts to get the sample from time `t` in channel `i`.
    pub fn get(&self, i: uint, t: Time) -> Option<T> {
        self.chs[i].borrow_mut().get(t)
    }

    /// Pushes the next sample to channel `i`.
    pub fn push(&self, i: uint, s: T) {
        self.chs[i].borrow_mut().push(s);
    }
}


/// Holds references to channels to draw input audio data from.
#[experimental]
pub struct InputArray<T> {
    chs: Vec<Option<ChannelRef<T>>>
}

impl<T: Clone+Default> InputArray<T> {
    /// Creates a new input array that can receive from `num_channels channels.
    /// 
    /// These channels are initialized as empty, and must be filled to return
    /// input data.
    pub fn new(num_channels: uint) -> InputArray<T> {
        let mut chs = Vec::with_capacity(num_channels);
        for _ in range(0, num_channels) {
            chs.push(None);
        }
        InputArray { chs: chs }
    }

    /// Returns the number of input channels
    pub fn get_num_channels(&self) -> uint {
        self.chs.len()
    }

    /// Sets channel `i` to read from `channel`.
    pub fn set_channel(&mut self, i: uint, channel: ChannelRef<T>) {
        self.chs[i] = Some(channel);
    }

    /// Sets channel `i` to empty.
    pub fn remove_channel(&mut self, i: uint) {
        self.chs[i] = None;
    }

    /// Attempts to return the value of channel `i` at time `t`.
    ///
    /// Returns `None` if either channel `i` is empty, or if channel `i` could
    /// not return the sample for time `t`.
    pub fn get(&self, i: uint, t: Time) -> Option<T> {
        match self.chs[i] {
            Some(ref ch) => ch.borrow().get(t),
            None => None
        }
    }
}

