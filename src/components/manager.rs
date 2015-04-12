//! Provides a manager class for device processing.

#![unstable]

use std::vec::Vec;

use types::{Device, Time};

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

    pub fn add_device(&mut self, device: &'a mut Device) -> usize {
        self.devices.push(device);
        self.devices.len()
    }

    pub fn tick(&mut self) {
        for device in self.devices.iter_mut() {
            device.tick(self.t);
        }
        self.t += 1;
    }
}
