use std::ops::{Deref, DerefMut};
use std::sync::mpsc::{channel, Sender, Receiver};

use types::{AudioDevice, MessageReceiver, Sample, Time};


/// Bundles a `MessageReceiver` with threaded message passing.
///
/// # Sending messages
///
/// Once a device is wrapped with `Messaged`, one or more `Sender`s can be
/// returned from it. These represent the send end of standard Rust channels,
/// and can be sent between threads freely.
///
/// # As an `AudioDevice`
///
/// The wrapper implements `AudioDevice` if the contained device does as well.
/// The implementation first checks for any incoming messages, then applies them
/// to the interior device. Once the messages have been handled, it calls the
/// normal `tick` function of the device.
///
/// This allows one to wrap a device in `Messaged`, move the sender to another
/// thread, then drop the `Messaged` device into a normal container, such as
/// `DeviceChain` or `DeviceGraph`. It will automatically receive and handle its
/// own messages while processing.
///
/// # Example
///
/// ```
/// use std::thread;
/// use oxcable::chain::{DeviceChain, Tick};
/// use oxcable::oscillator::{Oscillator, Sine, SetFreq};
/// use oxcable::io::audio::AudioEngine;
/// use oxcable::wrappers::Messaged;
///
/// fn main() {
/// # // this example uses tick_forever(), so we wrap it in a function that gets
/// # // compiled by not run by doctest
/// # }
/// # fn dummy() {
///     // Initialize signal chain...
///     let tx;
///     let engine = AudioEngine::with_buffer_size(256).unwrap();
///     let mut chain = DeviceChain::from({
///         let msgd = Messaged::from(Oscillator::new(Sine).freq(440.0));
///         tx = msgd.get_sender();
///         msgd
///     }).into(
///         engine.default_output(1).unwrap()
///     );
///
///     // Send the messager to a new thread. It will wait for one second, then
///     // set the oscillator to a higher frequency.
///     thread::spawn(move || {
///         thread::sleep_ms(1000);
///         tx.send(SetFreq(880.0));
///     });
///
///     // Generate two seconds of audio, then exit...
///     chain.tick_n_times(88200);
/// }
/// ```
pub struct Messaged<D> where D: MessageReceiver {
    /// The device being wrapped.
    pub device: D,
    tx: Sender<D::Msg>,
    rx: Receiver<D::Msg>,
}

impl<D> Messaged<D> where D: MessageReceiver {
    /// Return the sending half of our communication channel.
    pub fn get_sender(&self) -> Sender<D::Msg> {
        self.tx.clone()
    }
}

impl<D> From<D> for Messaged<D> where D: MessageReceiver {
    fn from(device: D) -> Self {
        let (tx, rx) = channel();
        Messaged {
            device: device,
            tx: tx,
            rx: rx,
        }
    }
}

impl<D> AudioDevice for Messaged<D> where D: AudioDevice+MessageReceiver {
    fn num_inputs(&self) -> usize {
        self.device.num_inputs()
    }

    fn num_outputs(&self) -> usize {
        self.device.num_outputs()
    }

    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        loop {
            match self.rx.try_recv() {
                Ok(msg) => self.device.handle_message(msg),
                _ => break
            }
        }
        self.device.tick(t, inputs, outputs);
    }
}

impl<D> Deref for Messaged<D> where D: AudioDevice+MessageReceiver {
    type Target = D;
    fn deref(&self) -> &D {
        &self.device
    }
}

impl<D> DerefMut for Messaged<D> where D: AudioDevice+MessageReceiver {
    fn deref_mut(&mut self) -> &mut D {
        &mut self.device
    }
}
