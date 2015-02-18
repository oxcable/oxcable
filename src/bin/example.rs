//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::init;
    use oxcable::types::Device;
    use oxcable::delay::Delay;
    use oxcable::io::audio::{AudioIn, AudioOut};
    use oxcable::mixers::Gain;
    use oxcable::oscillator;
    use oxcable::oscillator::Oscillator;

    println!("Setting up signal chain...");
    assert!(init::initialize().is_ok());

    let mut mic = AudioIn::new(1);
    let mut del = Delay::new(0.5, 0.5, 0.5, 1);
    del.inputs.set_channel(0, mic.outputs.get_channel(0));

    let mut osc = Oscillator::new(oscillator::Sine, 440.0);
    let mut gain = Gain::new(-12.0, 1);
    gain.inputs.set_channel(0, osc.output.get_channel());

    let mut spk = AudioOut::new(2);
    spk.inputs.set_channel(0, del.outputs.get_channel(0));
    spk.inputs.set_channel(1, gain.outputs.get_channel(0));

    println!("Playing...");
    let mut t = 0;
    loop {
        mic.tick(t);
        del.tick(t);
        osc.tick(t);
        gain.tick(t);
        spk.tick(t);
        t += 1;
    }
}
