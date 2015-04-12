//! Provides a manager class for device processing.

#![unstable]

use std::sync::mpsc::channel;
use std::thread;
use std::vec::Vec;

use types::{Device, Time, SAMPLE_RATE};

/// Tracks state and manages processing of many devices.
pub struct DeviceManager<'a> {
    t: Time,
    devices: Vec<&'a mut Device>
}

impl<'a> DeviceManager<'a> {
    /// Create an empty DeviceManager
    pub fn new() -> DeviceManager<'a> {
        DeviceManager {
            t: 0,
            devices: Vec::new()
        }
    }

    /// Captures a reference to the device for processing.
    ///
    /// Devices must be inserted in the order they should be processed. This
    /// means that a devices input needs to be added before that device.
    pub fn add_device(&mut self, device: &'a mut Device) -> usize {
        self.devices.push(device);
        self.devices.len()
    }

    /// Process a single time step.
    pub fn tick(&mut self) {
        for device in self.devices.iter_mut() {
            device.tick(self.t);
        }
        self.t += 1;
    }

    /// Tick until the user presses the enter key.
    pub fn loop_until_enter(&mut self) {
        let (tx, rx) = channel();
        let _ = thread::spawn(move || {
            use std::io::{Read, stdin};
            let mut buf = [0];
            let _ = stdin().read(&mut buf);
            assert!(tx.send(()).is_ok());
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
