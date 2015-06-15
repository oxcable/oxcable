//! Provides MIDI input from OS MIDI devices.

extern crate portmidi;

use std::io::{self, Read, Write};
use std::rc::Rc;

use types::{MidiDevice, MidiEvent, MidiMessage, Time};


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


/// This empty struct is used as a RAII marker for an initialized portmidi
/// connection. It is held in a Rc, and copies are passed to all streams opened
/// with it.
struct MidiEngineMarker;
impl Drop for MidiEngineMarker {
    fn drop(&mut self)
    {
        portmidi::terminate().unwrap();
    }
}

/// Used to handle portmidi resources.
pub struct MidiEngine {
    marker: Rc<MidiEngineMarker>
}

impl MidiEngine {
    pub fn open() -> Result<MidiEngine, portmidi::PortMidiError> {
        try!(portmidi::initialize());
        Ok(MidiEngine { marker: Rc::new(MidiEngineMarker) })
    }

    pub fn default_input(&self) -> MidiIn {
        MidiIn::new(self.marker.clone(),
            portmidi::get_default_input_device_id().unwrap())
    }

    pub fn choose_input(&self) -> MidiIn {
        println!("Select a MIDI input:");
        let default_in = portmidi::get_default_input_device_id();
        let mut valids = Vec::new();
        for i in 0..portmidi::count_devices() {
            match portmidi::get_device_info(i) {
                Some(device) => {
                    if device.input {
                        print!("   {}) {}", valids.len(), device.name);
                        if Some(device.device_id) == default_in {
                            print!(" (default)");
                        }
                        println!("");
                        valids.push(device);
                    }
                },
                _ => ()
            }
        }
        assert!(valids.len() > 0);

        let mut port = None;
        let mut s = String::new();
        while port.is_none() {
            print!(" > ");
            io::stdout().flush().unwrap();
            if io::stdin().read_line(&mut s).is_ok() {
                port = s.trim().parse::<usize>().ok().map_or(None, |i|
                    if i < valids.len() {
                        Some(i)
                    } else {
                        None
                    }
                );
            }
            s.clear();
        }

        MidiIn::new(self.marker.clone(),
                    port.unwrap() as portmidi::PortMidiDeviceId)
    }
}


/// Reads audio from the OS's default midi device.
pub struct MidiIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<MidiEngineMarker>,
    pm_stream: portmidi::InputPort,
}

impl MidiIn {
    /// Opens a midi input stream.
    fn new(engine: Rc<MidiEngineMarker>, port: portmidi::PortMidiDeviceId)
            -> MidiIn {
        // Open a stream. For now, use first device
        let mut pm_stream = portmidi::InputPort::new(port, BUFFER_SIZE);
        pm_stream.open().unwrap();

        MidiIn {
            engine: engine,
            pm_stream: pm_stream,
        }
    }

    /// Closes the portmidi stream
    pub fn stop(&mut self) {
        self.pm_stream.close().unwrap();
    }
}

impl MidiDevice for MidiIn {
    fn get_events(&mut self, t: Time) -> Vec<MidiEvent> {
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
