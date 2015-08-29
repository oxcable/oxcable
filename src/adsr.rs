//! An ADSR envelope filter.

use num::traits::Float;

use types::{SAMPLE_RATE, AudioDevice, MessageReceiver, Sample, Time};


/// Defines the messages that the ADSR supports
#[derive(Clone, Copy, Debug)]
pub enum Message {
    NoteDown,
    NoteUp,
    SetAttack(f32),
    SetDecay(f32),
    SetSustain(f32),
    SetRelease(f32),
}
pub use self::Message::*;


/// A multichannel ADSR filter
pub struct Adsr {
    // Remember parameter values
    num_channels: usize,
    attack_time: Time,
    decay_time: Time,
    release_time: Time,
    sustain_level: f32,

    // Track state
    current_state: AdsrState,
    next_state_change: Time,
    state_time: Time,
    level: f32,
    delta: f32,
    multiplier: f32,
}

impl Adsr {
    /// Returns a new ADSR filter with the provided envelope settings.
    ///
    /// * `attack_time` specifies the length of the attack in seconds.
    /// * `decay_time` specifies the length of the decay in seconds.
    /// * `sustain_level` specifies the amplitude of the sustain from 0 to 1.
    /// * `release_time` specifies the length of the release in seconds.
    /// * `num_channels` defines how many channels of audio to filter.
    pub fn new(attack_time: f32, decay_time: f32, sustain_level: f32,
               release_time: f32, num_channels: usize) -> Self {
        // Convert times to samples
        let attack_samples = (attack_time*SAMPLE_RATE as f32) as Time;
        let decay_samples = (decay_time*SAMPLE_RATE as f32) as Time;
        let release_samples = (release_time*SAMPLE_RATE as f32) as Time;

        Adsr {
            num_channels: num_channels,
            attack_time: attack_samples,
            decay_time: decay_samples,
            release_time: release_samples,
            sustain_level: sustain_level,
            current_state: AdsrState::Silent,
            next_state_change: 0,
            state_time: 0,
            level: 0.0,
            delta: 0.0,
            multiplier: 0.0,
        }
    }

    /// Returns an ADSR with reasonable default values for the envelope.
    pub fn default(num_channels: usize) -> Self {
        Adsr::new(0.05, 0.5, 0.5, 0.5, num_channels)
    }

    /// Triggers a state change and updates the corresponding state
    fn handle_state_change(&mut self, to: AdsrState) {
        match to {
            AdsrState::Attack => {
                self.current_state = AdsrState::Attack;
                self.next_state_change = self.attack_time;
                self.compute_deltas(1.0);
            },
            AdsrState::Decay => {
                self.current_state = AdsrState::Decay;
                self.next_state_change = self.decay_time;
                let goal = self.sustain_level;
                self.compute_deltas(goal);
            },
            AdsrState::Sustain => {
                self.current_state = AdsrState::Sustain;
                self.next_state_change = 0;
                self.level = self.sustain_level;
                self.delta = 0.0;
                self.multiplier = 0.0;
            },
            AdsrState::Release => {
                self.current_state = AdsrState::Release;
                self.next_state_change = self.release_time;
                self.compute_deltas(0.0);
            },
            AdsrState::Silent => {
                self.current_state = AdsrState::Silent;
                self.next_state_change = 0;
                self.level = 0.0;
                self.delta = 0.0;
                self.multiplier = 0.0;
            }
        }
        self.state_time = 0;
    }

    /// Compute the update parameters. Model the exponential envelope as
    /// an RC circuit
    fn compute_deltas(&mut self, dest: f32) {
        let tau = self.next_state_change as f32/4.0;
        self.multiplier = (-1.0/tau).exp();
        self.delta = (dest-self.level)*((1.0/tau).exp() - 1.0);
    }
}

impl MessageReceiver for Adsr {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        match msg {
            NoteDown => {
                self.handle_state_change(AdsrState::Attack);
            },
            NoteUp => {
                self.handle_state_change(AdsrState::Release);
            },
            SetAttack(attack) => {
                self.attack_time = (attack*SAMPLE_RATE as f32) as Time;
            },
            SetDecay(decay) => {
                self.decay_time = (decay*SAMPLE_RATE as f32) as Time;
            },
            SetSustain(sustain) => {
                self.sustain_level = sustain;
            },
            SetRelease(release) => {
                self.release_time = (release*SAMPLE_RATE as f32) as Time;
            },
        }
    }
}

impl AudioDevice for Adsr {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        // Handle any state changes
        if self.state_time == self.next_state_change {
            let next_state = self.current_state.next();
            self.handle_state_change(next_state);
        }
        self.state_time += 1;

        // Update the envelope
        self.level += self.delta;
        self.delta *= self.multiplier;

        // Apply the envelope
        for (i,s) in inputs.iter().enumerate() {
            outputs[i] = s*self.level;
        }
    }
}


/// Defines the current mode the ADSR is operating in
#[derive(Clone, Copy, Debug)]
enum AdsrState { Silent, Attack, Decay, Sustain, Release }

impl AdsrState {
    /// Given the current state, gets our next state
    fn next(self) -> Self {
        match self {
            AdsrState::Attack  => AdsrState::Decay,
            AdsrState::Decay   => AdsrState::Sustain,
            AdsrState::Sustain => AdsrState::Release,
            AdsrState::Release => AdsrState::Silent,
            AdsrState::Silent  => AdsrState::Silent
        }
    }
}
