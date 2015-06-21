//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::audio::AudioEngine;
    use oxcable::tick::tick_until_enter;

    println!("Initializing signal chain...");
    let engine = AudioEngine::open().unwrap();
    let mut chain = DeviceChain::from(
        engine.default_input(1)
    ).into(
        engine.default_output(1)
    );

    println!("Mirroring microphone input to speaker. Press Enter to quit.");
    tick_until_enter(&mut chain);
    println!("Done!");
}
