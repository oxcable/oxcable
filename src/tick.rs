use std::sync::mpsc::channel;
use std::thread;

use types::{SAMPLE_RATE, Time};

pub trait Tick {
    fn tick(&mut self);

    fn tick_n_times(&mut self, times: Time) {
        for _ in 0..times {
            self.tick();
        }
    }

    fn tick_forever(&mut self) {
        loop {
            self.tick();
        }
    }

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
