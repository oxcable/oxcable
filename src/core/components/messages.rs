//! Provides multithreaded message passing.
//!
//! A `Device` that wants to support message handling uses a `MessageReceiver`
//! component as an abstraction. The receiver can then hand out `MessageSender`
//! objects that reference the reciver. These senders can be sent to different
//! threads if required, and provide a safe interface to pass messages to their
//! receiver.
//!
//! The type and payload of the message is left up to the `Device` using this
//! component. However, the type must implement the `Clone`, `Send`, and `Sync`
//! traits. Further, since the payload will be copied multiple times, it is wise
//! to keep their size minimal.

#![experimental]

use std::sync::{Arc, Mutex};
use std::vec::Vec;


/// A component that receives messages
pub struct MessageReceiver<T> {
    msgs: Arc<Mutex<Vec<T>>>,
    num_senders: uint
}

impl<T: Clone+Send+Sync> MessageReceiver<T> {
    /// Creates a new receiver with no inputs
    pub fn new() -> MessageReceiver<T> {
        MessageReceiver { 
            msgs: Arc::new(Mutex::new(Vec::new())),
            num_senders: 0
        }
    }

    /// Returns a sender that writes messages only to this receiver
    pub fn get_sender(&mut self) -> MessageSender<T> {
        self.num_senders += 1;
        MessageSender { msgs: self.msgs.clone() }
    }

    /// Returns all the current messages and clears our input queue
    pub fn receive(&mut self) -> Vec<T> {
        if self.num_senders == 0 {
            Vec::new()
        } else {
            let mut msgs = self.msgs.lock().unwrap();
            let messages = msgs.clone();
            msgs.clear();
            messages
        }
    }
}


/// A component that sends messages to a single receiver.
pub struct MessageSender<T> {
    msgs: Arc<Mutex<Vec<T>>>
}

impl<T: Clone+Send+Sync> MessageSender<T> {
    /// Sends the provided message to our receiver.
    pub fn send(&mut self, msg: T) {
        self.msgs.lock().unwrap().push(msg.clone());
    }
}
