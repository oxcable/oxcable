use std::sync::mpsc::channel;
use std::thread;

use types::{SAMPLE_RATE, Time};

pub trait Tick {
    fn tick(&mut self);
}

pub fn tick_n_times<T>(ticker: &mut T, times: Time) where T: Tick {
    for _ in 0..times {
        ticker.tick();
    }
}

pub fn tick_forever<T>(ticker: &mut T) where T: Tick {
    loop {
        ticker.tick();
    }
}

pub fn tick_until_enter<T>(ticker: &mut T) where T: Tick {
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
            ticker.tick();
        }
        if rx.try_recv().is_ok() {
            break;
        }
    }
}
