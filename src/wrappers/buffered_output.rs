use std::ops::{Deref, DerefMut};

use types::{AudioDevice, Sample, Time};


/// Bundles an `AudioDevice` with an allocated output buffer.
///
/// To use the device, input samples are passed to `tick`. The output samples
/// can be found in the `outputs` buffer.
///
/// # Example
///
/// ```
/// use oxcable::types::{AudioDevice, Sample, Time};
/// struct IdentityFilter;
/// impl AudioDevice for IdentityFilter {
///     fn num_inputs(&self) -> usize { 1 }
///     fn num_outputs(&self) -> usize { 1 }
///     fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
///         outputs[0] = inputs[0];
///     }
/// }
///
/// use oxcable::wrappers::BufferedOutput;
/// let mut filter = BufferedOutput::from(IdentityFilter);
///
/// for i in 0..8 {
///     let input = [i as f32];
///     filter.tick(i, &input);
///     assert_eq!(i as f32, filter.outputs[0]);
/// }
/// ```
pub struct BufferedOutput<D> where D: AudioDevice {
    /// The AudioDevice being wrapped.
    pub device: D,
    /// The output buffer.
    pub outputs: Vec<Sample>,
}

impl<D> BufferedOutput<D> where D: AudioDevice {
    /// Calls the device's tick method using the wrapper's buffers.
    pub fn tick(&mut self, t: Time, inputs: &[Sample]) {
        self.device.tick(t, inputs, &mut self.outputs);
    }
}

impl<D> From<D> for BufferedOutput<D> where D: AudioDevice {
    fn from(device: D) -> Self {
        let outputs = device.num_outputs();
        BufferedOutput {
            device: device,
            outputs: vec![0.0; outputs],
        }
    }
}

impl<D> Deref for BufferedOutput<D> where D: AudioDevice {
    type Target = D;
    fn deref(&self) -> &D {
        &self.device
    }
}

impl<D> DerefMut for BufferedOutput<D> where D: AudioDevice {
    fn deref_mut(&mut self) -> &mut D {
        &mut self.device
    }
}
