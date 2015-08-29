use types::{AudioDevice, MessageReceiver, Sample, Time};


/// Defines the messages that the Multiplexer supports.
#[derive(Clone, Copy, Debug)]
pub enum Message {
    /// Selects the channel to pass to the output.
    SelectChannel(usize)
}
pub use self::Message::*;


/// A multiplexer.
///
/// A multiplexer takes in several inputs, and chooses only one of them to send
/// to the output. Since the input can be changed, this can be used as a switch
/// between multiple signals.
pub struct Multiplexer {
    num_inputs: usize,
    selected: usize,
}

impl Multiplexer {
    /// Returns a new multiplexer with `num_inputs` input and one output.
    pub fn new(num_inputs: usize) -> Self {
        Multiplexer {
            num_inputs: num_inputs,
            selected: 0
        }
    }

    /// Select the `i`th channel as the output. Returns Err if the channel is
    /// out of range.
    ///
    /// While this has identical behavior to the `SetChannel` message, the
    /// method is retained so that it may return a result.
    pub fn select_input(&mut self, i: usize) -> Result<(),()> {
        if i < self.num_inputs {
            self.selected = i;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl MessageReceiver for Multiplexer {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        let SelectChannel(i) = msg;
        let _ = self.select_input(i);
    }
}

impl AudioDevice for Multiplexer {
    fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        outputs[0] = inputs[self.selected];
    }
}
