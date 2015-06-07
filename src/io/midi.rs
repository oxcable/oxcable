//! Provides MIDI input from OS MIDI devices.

extern crate portmidi;

use std::rc::Rc;

use types::{MidiEvent, MidiMessage, Time};


/// Defines the maximum event buffer size for portmidi
static BUFFER_SIZE: i32 = 256;


/// Converts a raw portmidi message to an oxcable MIDI event
fn midievent_from_portmidi(event: portmidi::MidiEvent, t: Time) -> MidiEvent {
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

    MidiEvent { channel: channel, time: t, payload: payload }
}


/// Used to handle portaudio resources.
pub struct MidiEngine;

impl MidiEngine {
    pub fn open() -> Result<MidiEngine, portmidi::PortMidiError> {
        try!(portmidi::initialize());
        Ok(MidiEngine)
    }
}

impl Drop for MidiEngine {
    fn drop(&mut self)
    {
        assert!(portmidi::terminate().is_ok());
    }
}


/// Reads audio from the OS's default midi device.
pub struct MidiIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<MidiEngine>,
    pm_stream: portmidi::InputPort,
}

impl MidiIn {
    /// Opens a midi input stream.
    pub fn new(engine: Rc<MidiEngine>) -> MidiIn {
        // Open a stream. For now, use firs device
        let mut pm_stream = portmidi::InputPort::new(0, BUFFER_SIZE);
        assert!(pm_stream.open().is_ok());

        MidiIn {
            engine: engine,
            pm_stream: pm_stream,
        }
    }

    /// Closes the portmidi stream
    pub fn stop(&mut self) {
        assert!(self.pm_stream.close().is_ok());
    }

    pub fn get_events(&mut self, t: Time) -> Vec<MidiEvent> {
        let mut events = Vec::new();
        loop {
            match self.pm_stream.read().unwrap() {
                Some(pm_event) => events.push(midievent_from_portmidi(pm_event, t)),
                None => break
            }
        }
        events
    }
}
