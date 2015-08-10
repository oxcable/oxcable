//! MIDI input from system MIDI devices.
//!
//! A `MidiEngine` is used to manage the MIDI driver and open new MIDI streams.
//! All input streams must be opened through an engine instance.

use std::io::{self, Read, Write};
use std::rc::Rc;

use portmidi;

use types::{MidiDevice, MidiEvent, MidiMessage, Time};


/// Defines the maximum event buffer size for portmidi
static BUFFER_SIZE: i32 = 256;


/// A system resources manager.
pub struct MidiEngine {
    marker: Rc<MidiEngineMarker>
}

impl MidiEngine {
    /// Initialize the MIDI driver.
    pub fn open() -> Result<Self, MidiError> {
        try!(portmidi::initialize());
        Ok(MidiEngine { marker: Rc::new(MidiEngineMarker) })
    }

    /// Open a MidiIn using the default OS device.
    pub fn default_input(&self) -> Result<MidiIn, MidiError> {
        let device = try!(portmidi::get_default_input_device_id().ok_or(
                MidiError::NoDevices));
        MidiIn::new(self.marker.clone(), device)
    }

    /// Launch a command-line input selection message, then open a MidiIn using
    /// the user selected device.
    pub fn choose_input(&self) -> Result<MidiIn, MidiError> {
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
        if valids.len() == 0 { return Err(MidiError::NoDevices); }

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

        let port = port.unwrap() as portmidi::PortMidiDeviceId;
        MidiIn::new(self.marker.clone(), port)
    }
}


/// This empty struct is used as a RAII marker for an initialized portmidi
/// connection. It is held in a Rc, and copies are passed to all streams opened
/// with it. Once all the streams have been closed, and the engine falls out of
/// scope, then portmidi will automatically be terminated.
struct MidiEngineMarker;
impl Drop for MidiEngineMarker {
    fn drop(&mut self) {
        portmidi::terminate().unwrap();
    }
}


/// Read audio from the OS's default midi device.
pub struct MidiIn {
    #[allow(dead_code)] // the engine is used as an RAII marker
    engine: Rc<MidiEngineMarker>,
    pm_stream: portmidi::InputPort,
}

impl MidiIn {
    /// Open a midi input stream.
    fn new(engine: Rc<MidiEngineMarker>, port: portmidi::PortMidiDeviceId)
            -> Result<Self, MidiError> {
        // Open a stream. For now, use first device
        let mut pm_stream = portmidi::InputPort::new(port, BUFFER_SIZE);
        try!(pm_stream.open());

        Ok(MidiIn {
            engine: engine,
            pm_stream: pm_stream,
        })
    }
}

impl Drop for MidiIn {
    fn drop(&mut self) {
        // Close the portmidi stream
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


/// An error type for MIDI stream management
#[derive(Debug)]
pub enum MidiError {
    /// No MIDI devices can be found
    NoDevices,
    /// Wraps a PortMidi error
    PortMidi(portmidi::PortMidiError)
}

impl From<portmidi::PortMidiError> for MidiError {
    fn from(e: portmidi::PortMidiError) -> MidiError {
        MidiError::PortMidi(e)
    }
}


/// Converts a raw portmidi message to an oxcable MIDI event
fn midievent_from_portmidi(event: portmidi::MidiEvent, t: Time) -> MidiEvent {
    let msg = event.message;
    let channel = msg.status & 0x0F;
    let payload = match msg.status >> 4 {
        0b1000 => {
            let note = msg.data1;
            let velocity = (msg.data2 as f32) / 127.0;
            MidiMessage::NoteOff(note, velocity)
        },
        0b1001 => {
            let note = msg.data1;
            let velocity = (msg.data2 as f32) / 127.0;
            MidiMessage::NoteOn(note, velocity)
        },
        0b1110 => {
            let int_value = ((msg.data2 as i16) << 7) | (msg.data1 as i16);
            let bend = (int_value - 0x2000) as f32 /
                (0x2000i16) as f32;
            MidiMessage::PitchBend(bend)
        },
        0b1010 => {
            let note = msg.data1;
            let pressure = (msg.data2 as f32) / 127.0;
            MidiMessage::KeyPressure(note, pressure)
        },
        0b1011 => match msg.data1 {
            0x40 => MidiMessage::SustainPedal(msg.data2 >= 64),
            _ => MidiMessage::ControlChange(msg.data1, msg.data2)
        },
        0b1100 => MidiMessage::ProgramChange(msg.data1),
        0b1101 => MidiMessage::ChannelPressure(msg.data1 as f32 / 127.0),
        _ => MidiMessage::Other(msg.status, msg.data1, msg.data2)
    };

    MidiEvent { channel: channel, time: t, payload: payload }
}
