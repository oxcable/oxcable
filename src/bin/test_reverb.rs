//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::rc::Rc;
    use oxcable::types::Device;
    use oxcable::io::audio::{AudioEngine, AudioIn, AudioOut};
    use oxcable::reverb::{MoorerReverb, rooms};

    println!("Initializing signal chain...");
    let engine = Rc::new(AudioEngine::open().unwrap());

    let mut mic = AudioIn::new(engine.clone(), 1);
    let mut rev = MoorerReverb::new(rooms::hall(), 1.0, -3.0, 0.5, 1);
    let mut spk = AudioOut::new(engine.clone(), 1);
    rev.inputs.set_channel(0, mic.outputs.get_channel(0));
    spk.inputs.set_channel(0, rev.outputs.get_channel(0));

    println!("Playing...");
    let mut t = 0;
    loop {
        mic.tick(t);
        rev.tick(t);
        spk.tick(t);
        t += 1;
    }
}
