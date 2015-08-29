use types::{AudioDevice, MessageReceiver, Sample, Time};
use utils::helpers::decibel_to_ratio;
use dynamics::level_detector::LevelDetector;


/// Defines the messages that the NoiseGate supports.
#[derive(Clone, Copy, Debug)]
pub enum Message {
    /// Sets the threshold to open the gate, in decibels.
    SetOnThreshold(f32),
    /// Sets the threshold to close the gate, in decibels.
    SetOffThreshold(f32),
    /// Sets the output gian, in decibels.
    SetGain(f32)
}
pub use self::Message::*;


/// A noise gate.
///
/// Noise gates provides a floor the signal must exceed; anything below the
/// floor is muted instead.
///
/// The noise gate provides two different thresholds. The gate will open once
/// the signal exceeds the on threshold, and close when the signal level drops
/// below the off threshold.
pub struct NoiseGate {
    level_detectors: Vec<LevelDetector>,
    active: bool,
    num_channels: usize,
    on_threshold: f32,
    off_threshold: f32,
    gain: f32
}

impl NoiseGate {
    /// Creates a new compressor.
    ///
    /// The `on_threshold` and `off_threshold` specify the signal floors in
    /// decibels.
    ///
    /// The specified `gain` (in decibels) will be applied to the signal after
    /// compression.
    pub fn new(on_threshold: f32, off_threshold: f32, gain: f32,
               num_channels: usize) -> Self {
        NoiseGate {
            level_detectors: vec![LevelDetector::default(); num_channels],
            active: false,
            num_channels: num_channels,
            on_threshold: decibel_to_ratio(on_threshold),
            off_threshold: decibel_to_ratio(off_threshold),
            gain: decibel_to_ratio(gain)
        }
    }
}

impl MessageReceiver for NoiseGate {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        match msg {
            SetOnThreshold(threshold) => {
                self.on_threshold = decibel_to_ratio(threshold);
            },
            SetOffThreshold(threshold) => {
                self.off_threshold = decibel_to_ratio(threshold);
            },
            SetGain(gain) => {
                self.gain = decibel_to_ratio(gain);
            }
        }
    }
}

impl AudioDevice for NoiseGate {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            // Look for a state change
            let level = self.level_detectors[i].compute_next_level(*s);
            if self.active && level < self.off_threshold {
                self.active = false;
            } else if !self.active && level > self.on_threshold {
                self.active = true;
            }

            // Gate the signal
            outputs[i] = if self.active { self.gain*s } else { 0.0 };
        }
    }
}
