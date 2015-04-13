//! Provides a time synchronous container for data.
//!
//! A Channel provides single-writer, multiple-reader, thread-local storage of
//! time-marked data.

#![unstable]

extern crate test;

use std::default::Default;

use types::Time;


/// Container for a single channel of data.
pub struct Channel<T> {
    next_t: Time,
    data: T,
}

impl<T: Clone+Default> Channel<T> {
    /// Returns a new channel starting at time `t=0`.
    pub fn new() -> Channel<T> {
        Channel{ next_t: 0, data: Default::default() }
    }

    /// Attempts to return the data frame for time `t`.
    ///
    /// Returns `None` if we don't have the requested time.
    #[inline]
    pub fn get(&self, t: Time) -> Option<T> {
        if t != self.next_t-1 {
            None
        } else {
            Some(self.data.clone())
        }
    }

    /// Add the next frame to the channel.
    #[inline]
    pub fn push(&mut self, f: T) {
        self.data = f.clone();
        self.next_t += 1;
    }

    /// Returns a read only reference to this channel
    pub fn get_reader(&self) -> ChannelRef<T> {
        ChannelRef { ch: self }
    }
}


/// A read-only reference to a channel
pub struct ChannelRef<T>  {
    ch: *const Channel<T>
}

impl<T: Clone+Default> ChannelRef<T> {
    #[inline]
    pub fn get(&self, t: Time) -> Option<T> {
        unsafe {
            (*self.ch).get(t)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use super::{Channel};

    #[bench]
    fn bench_ref_get(b: &mut Bencher) {
        let mut writer = Channel::new();
        let reader = writer.get_reader();
        writer.push(0.0);
        b.iter(|| reader.get(0));
    }

    #[bench]
    fn bench_ref_push(b: &mut Bencher) {
        let mut writer = Channel::new();
        b.iter(|| writer.push(0.0));
    }
}
