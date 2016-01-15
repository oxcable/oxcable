use std::thread::panicking;

use types::{AudioDevice, Sample, Time};

/// Defines a mock AudioDevice implementation for testing.
///
/// The mock device will panic if its `tick` method is called without expecting
/// a call, or if it does not receive the expected input frame.
///
/// The mock device will panic on deconstruct if it was not ticked enough times.
pub struct MockAudioDevice {
    name: String,
    num_inputs: usize,
    num_outputs: usize,
    times_ticked: usize,
    inputs: Vec<Vec<Sample>>,
    outputs: Vec<Vec<Sample>>,
    failed: bool,
}

impl MockAudioDevice {
    /// Creates a new MockAudioDevice.
    ///
    /// `name` will be used to print more helpful error messages.
    pub fn new(name: &str, inputs: usize, outputs: usize) -> MockAudioDevice {
        MockAudioDevice {
            name: String::from(name),
            num_inputs: inputs,
            num_outputs: outputs,
            times_ticked: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            failed: false,
        }
    }

    /// Adds expected input and output for the next tick.
    pub fn will_tick(&mut self, inputs: &[Sample], outputs: &[Sample])
            -> &mut Self {
        self.inputs.push(Vec::from(inputs));
        self.outputs.push(Vec::from(outputs));
        self
    }
}

impl Drop for MockAudioDevice {
    fn drop(&mut self) {
        if !self.failed && !panicking() &&
                self.times_ticked != self.inputs.len() {
            panic!("MockAudioDevice \"{}\" ticked {} times \
                    (expected {} times)",
                    self.name, self.inputs.len(), self.times_ticked);
        }
    }
}

impl AudioDevice for MockAudioDevice {
    fn num_inputs(&self) -> usize { self.num_inputs }
    fn num_outputs(&self) -> usize { self.num_outputs }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        if self.times_ticked >= self.inputs.len() {
            self.failed = true;
            panic!("MockAudioDevice \"{}\" ticked {} times \
                    (expected {} times)",
                    self.name, self.inputs.len(), self.times_ticked+1);
        }
        if self.inputs[self.times_ticked] != inputs {
            self.failed = true;
            panic!("MockAudioDevice \"{}\" got wrong input. \
                    Expected: {:?}, got: {:?}",
                    self.name, self.inputs[self.times_ticked], inputs);
        }
        for (i, output) in self.outputs[self.times_ticked].iter().enumerate() {
            outputs[i] = *output;
        }
        self.times_ticked += 1;
    }
}

#[cfg(test)]
mod test_mock_audio_device {
    use types::AudioDevice;
    use super::MockAudioDevice;

    #[test]
    fn test_success() {
        let mut buffer = [0.0];
        let mut mock = MockAudioDevice::new("mock", 1, 1);

        mock.will_tick(&[1.0], &[1.0]);
        mock.tick(0, &[1.0], &mut buffer);
        assert_eq!(buffer, [1.0]);

        mock.will_tick(&[2.0], &[3.0]);
        mock.tick(1, &[2.0], &mut buffer);
        assert_eq!(buffer, [3.0]);
    }

    #[test]
    #[should_panic]
    fn test_too_few_ticks() {
        let mut mock = MockAudioDevice::new("mock", 1, 0);
        mock.will_tick(&[0.0], &[]);
    }

    #[test]
    #[should_panic]
    fn test_too_many_ticks() {
        let mut mock = MockAudioDevice::new("mock", 1, 0);
        mock.tick(0, &[0.0], &mut[]);
    }

    #[test]
    #[should_panic]
    fn test_wrong_input() {
        let mut mock = MockAudioDevice::new("mock", 1, 0);
        mock.will_tick(&[0.0], &[]);
        mock.tick(0, &[1.0], &mut[]);
    }
}
