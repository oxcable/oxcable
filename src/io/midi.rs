//! Provides MIDI input from OS MIDI devices.

#![experimental]

extern crate portmidi;

use std::vec::Vec;

use core::types::{Device, MidiEvent, Time};
use core::components::OutputElement;
use core::init;


/// Defines the maximum event buffer size for portmidi
static BUFFER_SIZE: int = 256;


fn midievent_from_portmidi(event: portmidi::midi::PmEvent) -> MidiEvent {
    MidiEvent { 
        status: event.message.status as u8,
        byte1: event.message.data1 as u8,
        byte2: event.message.data2 as u8
    }
}


/// Reads audio from the OS's default midi device.
pub struct MidiIn {
    /// Output midi channel
    pub output: OutputElement<Vec<MidiEvent>>,

    pm_stream: portmidi::midi::PmInputPort,
}

impl MidiIn {
    /// Opens a midi input stream.
    pub fn new() -> MidiIn {
        // Check for initialization
        if !init::is_initialized() {
            panic!("Must initialize oxcable first");
        }
        
        // Open a stream. For now, use firs device
        let mut pm_stream = portmidi::midi::PmInputPort::new(1, BUFFER_SIZE);
        assert_eq!(pm_stream.open(), portmidi::midi::PmError::PmNoError);

        MidiIn {
            output: OutputElement::new(),
            pm_stream: pm_stream,
        }
    }

    /// Closes the portmidi stream
    pub fn stop(&mut self) {
        assert_eq!(self.pm_stream.close(), portmidi::midi::PmError::PmNoError);
    }
}

impl Device for MidiIn {
    fn tick(&mut self, _t: Time) {
        let mut events = Vec::new();
        while self.pm_stream.poll() == portmidi::midi::PmError::PmGotData {
            let pm_message = self.pm_stream.read().unwrap();
            let event = midievent_from_portmidi(pm_message);
            events.push(event);
        }
        self.output.push(events);
    }
}
