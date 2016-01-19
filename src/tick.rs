//! A trait for objects that process in discrete time steps.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use types::{SAMPLE_RATE, Time};


/// Methods for processing something in discrete time steps. By defining how to
/// perform a single tick, this trait gives several convenience methods.
///
/// A single tick could correspond to one frame, but it doesn't have to. For
/// example, if an object does buffering it may process many samples per tick.
pub trait Tick {
    /// Handles one time step.
    fn tick(&mut self);

    /// Runs `tick` `n` times. Returns after processing.
    fn tick_n_times(&mut self, n: Time) {
        for _ in 0..n {
            self.tick();
        }
    }

    /// Ticks into infinity. Never returns.
    fn tick_forever(&mut self) -> ! {
        loop {
            self.tick();
        }
    }

    /// Ticks while waiting for the user to press `Enter`. When enter is
    /// pressed, ticking stops and the method returns.
    fn tick_until_enter(&mut self) {
        let stopped = Arc::new(AtomicBool::new(false));
        let signal_stop = stopped.clone();
        thread::spawn(move || {
            use std::io::{Read, stdin};
            let mut buf = [0];
            let _ = stdin().read(&mut buf);
            signal_stop.store(true, Ordering::Relaxed);
        });

        let ticks = SAMPLE_RATE / 10;
        loop {
            // Tick for 100ms, then check for exit command
            for _ in 0..ticks {
                self.tick();
            }
            if stopped.load(Ordering::Relaxed) {
                break;
            }
        }
    }
}
