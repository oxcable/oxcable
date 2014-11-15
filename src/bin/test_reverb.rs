//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::core::AudioDevice;
    use oxcable::io::microphone::Microphone;
    use oxcable::io::speaker::Speaker;
    use oxcable::reverb::{MoorerReverb, rooms};
    println!("Playing...");

    let mut mic = Microphone::new(1);
    let mut rev = MoorerReverb::new(rooms::hall(), 1.0, -3.0, 0.5, 1);
    let mut spk = Speaker::new(1);
    rev.inputs.set_channel(0, mic.outputs.get_channel(0));
    spk.inputs.set_channel(0, rev.outputs.get_channel(0));

    let mut t = 0;
    loop {
        mic.tick(t);
        rev.tick(t);
        spk.tick(t);
        t += 1;
    }
}
