//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::rc::Rc;
    use oxcable::chain::DeviceChain;
    use oxcable::io::audio::{AudioEngine, AudioIn, AudioOut};

    println!("Initializing signal chain...");
    let engine = Rc::new(AudioEngine::open().unwrap());
    let mut chain = DeviceChain::from(AudioIn::new(engine.clone(), 1))
        .into(AudioOut::new(engine.clone(), 1));

    println!("Mirroring microphone input to speaker...");
    loop {
        chain.tick();
    }
}
