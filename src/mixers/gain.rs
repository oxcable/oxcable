use types::{AudioDevice, MessageReceiver, Sample, Time};
use utils::helpers::decibel_to_ratio;


/// Defines the messages that the Gain supports
#[derive(Clone, Copy, Debug)]
pub enum Message {
    SetGain(f32)
}
pub use self::Message::*;


/// A gain filter.
pub struct Gain {
    num_channels: usize,
    gain: f32,
}

impl Gain {
    /// Returns a new gain filter.
    ///
    /// `gain` is in decibels.
    pub fn new(gain: f32, num_channels: usize) -> Self {
        Gain {
            num_channels: num_channels,
            gain: decibel_to_ratio(gain)
        }
    }
}

impl MessageReceiver for Gain {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        let SetGain(gain) = msg;
        self.gain = decibel_to_ratio(gain);
    }
}

impl AudioDevice for Gain {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            outputs[i] = self.gain*s;
        }
    }
}
