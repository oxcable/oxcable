//! A trait for objects that process in discrete time steps.

use std::sync::mpsc::channel;
use std::thread;

use types::{SAMPLE_RATE, Time};


/// Methods for processing something in discrete time steps.
///
/// The implementor must only implement the `tick` method.  By defining `tick`
/// this trait provides several more convenience methods for controlling time.
pub trait Tick {
    /// Handles a single time step.
    fn tick(&mut self);

    /// Runs `tick` `n` times. Returns after processing.
    fn tick_n_times(&mut self, n: Time) {
        for _ in 0..n {
            self.tick();
        }
    }

    /// Ticks into infinity. Never returns.
    fn tick_forever(&mut self) {
        loop {
            self.tick();
        }
    }

    /// Ticks while waiting for the user to press `Enter`. When enter is
    /// pressed, ticking stops and the method returns.
    fn tick_until_enter(&mut self) {
        let (tx, rx) = channel();
        let _ = thread::spawn(move || {
            use std::io::{Read, stdin};
            let mut buf = [0];
            let _ = stdin().read(&mut buf);
            tx.send(()).unwrap();
        });

        let ticks = SAMPLE_RATE / 10;
        loop {
            // Tick for 100ms, then check for exit command
            for _ in 0..ticks {
                self.tick();
            }
            if rx.try_recv().is_ok() {
                break;
            }
        }
    }
}
