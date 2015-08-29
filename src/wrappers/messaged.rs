use std::ops::{Deref, DerefMut};
use std::sync::mpsc::{channel, Sender, Receiver};

use types::{AudioDevice, MessageReceiver, Sample, Time};


/// Bundles a MessageReceiver with threaded message passing.
pub struct Messaged<D> where D: MessageReceiver {
    /// The device being wrapped.
    pub device: D,
    tx: Sender<D::Msg>,
    rx: Receiver<D::Msg>,
}

impl<D> Messaged<D> where D: MessageReceiver {
    /// Return the sending half of our communication channel.
    pub fn get_sender(&mut self) -> Sender<D::Msg> {
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

impl<D> Deref for Messaged<D> where D: MessageReceiver {
    type Target = D;
    fn deref(&self) -> &D {
        &self.device
    }
}

impl<D> DerefMut for Messaged<D> where D: MessageReceiver {
    fn deref_mut(&mut self) -> &mut D {
        &mut self.device
    }
}
