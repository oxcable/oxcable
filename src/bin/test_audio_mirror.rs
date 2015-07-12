//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::audio::AudioEngine;
    use oxcable::tick::Tick;

    println!("Initializing signal chain...");
    let engine = AudioEngine::with_buffer_size(128).unwrap();
    let mut chain = DeviceChain::from(
        engine.default_input(1).unwrap()
    ).into(
        engine.default_output(1).unwrap()
    );

    println!("Mirroring microphone input to speaker. Press Enter to quit.");
    chain.tick_until_enter();
    println!("Done!");
}
