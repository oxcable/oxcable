//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::core::AudioDevice;
    use oxcable::io::microphone::Microphone;
    use oxcable::io::speaker::Speaker;
    println!("Mirroring microphone input to speaker...");

    let mut mic = Microphone::new(1);
    let mut spk = Speaker::new(1);
    spk.inputs.set_channel(0, mic.outputs.get_channel(0));

    let mut t: oxcable::core::Time = 0;
    loop {
        mic.tick(t);
        spk.tick(t);
        t += 1;
    }
}
