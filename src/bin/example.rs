//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::core::AudioDevice;
    use oxcable::delay::Delay;
    use oxcable::io::microphone::Microphone;
    use oxcable::io::speaker::Speaker;
    use oxcable::mixers::Gain;
    use oxcable::oscillator;
    use oxcable::oscillator::Oscillator;
    println!("Playing...");

    let mut mic = Microphone::new(1);
    let mut del = Delay::new(0.5, 0.5, 0.5, 1);
    del.inputs.set_channel(0, mic.outputs.get_channel(0));

    let mut osc = Oscillator::new(oscillator::Sine, 440.0);
    let mut gain = Gain::new(-12.0, 1);
    gain.inputs.set_channel(0, osc.output.get_channel(0));

    let mut spk = Speaker::new(2);
    spk.inputs.set_channel(0, del.outputs.get_channel(0));
    spk.inputs.set_channel(1, gain.outputs.get_channel(0));

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
