//! Provides MIDI input from OS MIDI devices.

#![unstable]

extern crate portmidi;

use std::vec::Vec;

use types::{Device, MidiEvent, MidiMessage, Time};
use components::OutputElement;
use init;


/// Defines the maximum event buffer size for portmidi
static BUFFER_SIZE: i32 = 256;


/// Converts a raw portmidi message to an oxcable MIDI event
fn midievent_from_portmidi(event: portmidi::MidiEvent) -> MidiEvent {
    let msg = event.message;
    let channel = (msg.status & 0x0F) as u8;
    let payload = match (msg.status as u8) >> 4 {
        0b1000 => {
            let note = msg.data1 as u8;
            let velocity = (msg.data2 as f32) / 127.0;
            MidiMessage::NoteOff(note, velocity)
        },
        0b1001 => {
            let note = msg.data1 as u8;
            let velocity = (msg.data2 as f32) / 127.0;
            MidiMessage::NoteOn(note, velocity)
        }
        0b1110 => {
            let int_value = ((msg.data2 as i16) << 7) | (msg.data1 as i16);
            let bend = (int_value - 0x2000) as f32 / 
                (0x2000i16) as f32;
            MidiMessage::PitchBend(bend)
        }
        0b1010 => {
            let note = msg.data1 as u8;
            let pressure = (msg.data2 as f32) / 127.0;
            MidiMessage::KeyPressure(note, pressure)
        }
        0b1011 => MidiMessage::ControlChange(msg.data1 as u8, msg.data2 as u8),
        0b1100 => MidiMessage::ProgramChange(msg.data1 as u8),
        0b1101 => MidiMessage::ChannelPressure(msg.data1 as f32 / 127.0),
        _ => MidiMessage::Other(msg.status as u8, msg.data1 as u8, 
                                msg.data2 as u8)
    };

    MidiEvent { channel: channel, payload: payload }
}


/// Reads audio from the OS's default midi device.
#[stable]
pub struct MidiIn {
    /// Output midi channel
    #[stable]
    pub output: OutputElement<Vec<MidiEvent>>,

    pm_stream: portmidi::InputPort,
}

#[stable]
impl MidiIn {
    /// Opens a midi input stream.
    #[stable]
    pub fn new() -> MidiIn {
        // Check for initialization
        if !init::is_initialized() {
            panic!("Must initialize oxcable first");
        }
        
        // Open a stream. For now, use firs device
        let mut pm_stream = portmidi::InputPort::new(1, BUFFER_SIZE);
        assert!(pm_stream.open().is_ok());

        MidiIn {
            output: OutputElement::new(),
            pm_stream: pm_stream,
        }
    }

    /// Closes the portmidi stream
    #[stable]
    pub fn stop(&mut self) {
        assert!(self.pm_stream.close().is_ok());
    }
}

impl Device for MidiIn {
    fn tick(&mut self, _t: Time) {
        let mut events = Vec::new();
        loop {
            match self.pm_stream.read().unwrap() {
                Some(pm_event) => 
                    events.push(midievent_from_portmidi(pm_event)),
                None => break
            }
        }
        self.output.push(events);
    }
}
