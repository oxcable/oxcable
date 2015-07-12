//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::audio::AudioEngine;
    use oxcable::reverb::{MoorerReverb, rooms};
    use oxcable::tick::Tick;

    println!("Initializing signal chain...");
    let engine = AudioEngine::with_buffer_size(256).unwrap();
    let mut chain = DeviceChain::from(
        engine.default_input(1).unwrap()
    ).into(
        MoorerReverb::new(rooms::HALL, 1.0, -3.0, 0.5, 1)
    ).into(
        engine.default_output(1).unwrap()
    );

    println!("Playing... Press Enter to quit.");
    chain.tick_until_enter();
    println!("Done!");
}
