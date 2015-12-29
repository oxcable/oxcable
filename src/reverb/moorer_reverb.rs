//! An algorithmic, IIR reverberation filter.

use num::traits::Float;

use types::{SAMPLE_RATE, AudioDevice, MessageReceiver, Sample, Time};
use utils::ringbuffer::RingBuffer;
use utils::helpers::decibel_to_ratio;
use reverb::rooms::Room;


/// Defines the messages that the MoorerReverb supports.
#[derive(Clone, Copy, Debug)]
pub enum Message {
    /// Sets the length of the reverberation, in seconds.
    SetReverbTime(f32),
    /// Sets the feedback level; should be between 0.0 and 1.0.
    SetGain(f32),
    /// Sets the wetness level; should be between 0.0 and 1.0.
    SetWetness(f32),
}
pub use self::Message::*;


/// An algorithmic, IIR reverberation filter.
///
/// This algorithmic reverb filter follows the basic design specified by James
/// Moorer in his seminal paper, "About This Reverberation Business".
pub struct MoorerReverb {
    num_channels: usize,
    gain: f32,
    wetness: f32,

    tapped_delay_lines: Vec<RingBuffer<Sample>>,
    tapped_delays: Vec<Time>,
    tapped_gains: Vec<f32>,

    comb_delay_lines: Vec<Vec<RingBuffer<Sample>>>,
    comb_delays: Vec<Time>,
    comb_gains: Vec<f32>,
    comb_out_buffer: Vec<RingBuffer<Sample>>,
    comb_out_gain: f32,
}

impl MoorerReverb {
    /// Creates and initializes a new reverberator.
    ///
    /// * `room`: a [`Room`](rooms/struct.Room.html) struct. This specifies
    ///   aspects of the reverberation character.
    /// * `rev_time`: the -60dB time, in seconds.
    /// * `gain`: the output gain, in decibels.
    /// * `wetness`: how much of the input signal to mix into the output.
    /// * `num_channels`: number of channels to process.
    pub fn new(room: Room, rev_time: f32, gain: f32, wetness:f32,
           num_channels: usize) -> Self {
        assert!(room.tapped_delays.len() == room.tapped_gains.len());

        // Calculate delays
        let max_tapped_delay = room.tapped_delays[room.tapped_delays.len()-1];
        let comb_out_delay = max_tapped_delay;

        // Calculate comb gains based on reverberation time
        let (comb_gains, comb_out_gain) =
            compute_comb_gains(&room.comb_delays, rev_time);

        // Allocate tapped delay lines
        let init = vec![0.0; max_tapped_delay as usize + 1];
        let tapped_delay_lines = vec![RingBuffer::from(&init[..]); num_channels];

        // Allocate comb delay lines
        let mut comb_delay_lines = Vec::with_capacity(num_channels);
        for _ in 0..num_channels {
            let mut channel_lines = Vec::with_capacity(room.comb_delays.len());
            for j in 0..room.comb_delays.len() {
                let delay = room.comb_delays[j];
                let init = vec![0.0; delay as usize + 1];
                channel_lines.push(RingBuffer::from(&init[..]));
            }
            comb_delay_lines.push(channel_lines);
        }

        let init = vec![0.0; comb_out_delay as usize + 1];
        let comb_out_buffer = vec![RingBuffer::from(&init[..]); num_channels];

        // Return struct
        MoorerReverb {
            num_channels: num_channels,
            gain: decibel_to_ratio(gain),
            wetness: wetness,
            tapped_delay_lines: tapped_delay_lines,
            tapped_delays: Vec::from(room.tapped_delays),
            tapped_gains: Vec::from(room.tapped_gains),
            comb_delay_lines: comb_delay_lines,
            comb_delays: Vec::from(room.comb_delays),
            comb_gains: comb_gains,
            comb_out_buffer: comb_out_buffer,
            comb_out_gain: comb_out_gain,
        }
    }
}

impl MessageReceiver for MoorerReverb {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        match msg {
            SetReverbTime(rev_time) => {
                let (comb_gains, comb_out_gain) =
                    compute_comb_gains(&self.comb_delays, rev_time);
                self.comb_gains = comb_gains;
                self.comb_out_gain = comb_out_gain;
            },
            SetGain(gain) => {
                self.gain = decibel_to_ratio(gain);
            },
            SetWetness(wetness) => {
                self.wetness = wetness;
            }
        }
    }
}

impl AudioDevice for MoorerReverb {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,x) in inputs.iter().enumerate() {
            // Advance tapped delay line
            for (delay, gain) in self.tapped_delays.iter()
                    .zip(self.tapped_gains.iter()) {
                self.tapped_delay_lines[i][t + *delay] += *gain * x;
            }
            let tapped_out = (x + self.tapped_delay_lines[i][t]) /
                (self.tapped_delays.len() as f32);
            self.tapped_delay_lines[i].push(0.0);

            // Update comb filters
            let comb_out = self.comb_out_buffer[i][t];
            let mut next_comb_out = 0.0;
            for j in 0..self.comb_delays.len() {
                let gain  = self.comb_gains[j];
                let feedback = self.comb_delay_lines[i][j][t];
                self.comb_delay_lines[i][j].push(tapped_out + gain * feedback);
                next_comb_out += feedback;
            }
            next_comb_out *= self.comb_out_gain/(self.comb_delays.len() as f32);
            self.comb_out_buffer[i].push(next_comb_out);

            // Compute and store result
            let wet_out = tapped_out + comb_out;
            let y = self.gain * (self.wetness*wet_out + (1.0-self.wetness)*x);
            outputs[i] = y;
        }
    }
}

/// Calculates comb gains based on reverberation time
///
/// Inputs:
///  * comb_delays: the delays for each comb filter, in samples
///  * rev_time: the reverberation time, in seconds
///
/// Outputs:
///  * A vector of gains for each comb filter
///  * The final output gain of the comb filter summation
fn compute_comb_gains(comb_delays: &[Time], rev_time: f32)
        -> (Vec<f32>, f32) {
    let mut comb_gains = Vec::with_capacity(comb_delays.len());
    for &delay in comb_delays.iter() {
        let gain = 10.0.powf(-3.0*(delay as f32)*rev_time / (SAMPLE_RATE as f32));
        comb_gains.push(gain);
    }
    let comb_out_gain = 1.0 - 0.366/rev_time;
    (comb_gains, comb_out_gain)
}
