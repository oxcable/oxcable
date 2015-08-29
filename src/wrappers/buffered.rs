use std::ops::{Deref, DerefMut};

use types::{AudioDevice, Sample, Time};


/// Bundles an `AudioDevice` with allocated input and output buffers.
///
/// To use the device, input samples must first be manually dropped into the
/// `inputs` buffer, then `tick` may be called to generate outputs. The output
/// samples can be found in the `outputs` buffer.
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
/// use oxcable::wrappers::Buffered;
/// let mut filter = Buffered::from(IdentityFilter);
///
/// for i in 0..8 {
///     filter.inputs[0] = i as f32;
///     filter.tick(i);
///     assert_eq!(i as f32, filter.outputs[0]);
/// }
/// ```
pub struct Buffered<D> where D: AudioDevice {
    /// The AudioDevice being wrapped.
    pub device: D,
    /// The input buffer.
    pub inputs: Vec<Sample>,
    /// The output buffer.
    pub outputs: Vec<Sample>,
}

impl<D> Buffered<D> where D: AudioDevice {
    /// Calls the device's tick method using the wrapper's buffers.
    pub fn tick(&mut self, t: Time) {
        self.device.tick(t, &self.inputs, &mut self.outputs);
    }
}

impl<D> From<D> for Buffered<D> where D: AudioDevice {
    fn from(device: D) -> Self {
        let inputs = device.num_inputs();
        let outputs = device.num_outputs();
        Buffered {
            device: device,
            inputs: vec![0.0; inputs],
            outputs: vec![0.0; outputs],
        }
    }
}

impl<D> Deref for Buffered<D> where D: AudioDevice {
    type Target = D;
    fn deref(&self) -> &D {
        &self.device
    }
}

impl<D> DerefMut for Buffered<D> where D: AudioDevice {
    fn deref_mut(&mut self) -> &mut D {
        &mut self.device
    }
}
