//! Provides a time synchronous container for data.

#![unstable]

extern crate test;

use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

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
}


/// A reference to a channel, used to link outputs to inputs.
#[derive(Clone)]
pub struct ChannelRef<T>  {
    ch: Rc<RefCell<Channel<T>>>
}

impl<T: Clone+Default> ChannelRef<T> {
    pub fn new() -> ChannelRef<T> {
        ChannelRef {
            ch: Rc::new(RefCell::new(Channel::new()))
        }
    }

    #[inline]
    pub fn get(&self, t: Time) -> Option<T> {
        self.ch.borrow().get(t)
    }

    #[inline]
    pub fn push(&self, f: T) {
        self.ch.borrow_mut().push(f);
    }
}


#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use core::types::Sample;
    use super::ChannelRef;

    #[bench]
    fn bench_ref_get(b: &mut Bencher) {
        let ch: ChannelRef<Sample> = ChannelRef::new();
        ch.push(0.0);
        b.iter(|| ch.get(0));
    }

    #[bench]
    fn bench_ref_push(b: &mut Bencher) {
        let ch: ChannelRef<Sample> = ChannelRef::new();
        b.iter(|| ch.push(0.0));
    }
}
