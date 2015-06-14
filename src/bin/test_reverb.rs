//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::audio::AudioEngine;
    use oxcable::reverb::{MoorerReverb, rooms};
    use oxcable::utils::tick::tick_until_enter;

    println!("Initializing signal chain...");
    let engine = AudioEngine::open().unwrap();
    let mut chain = DeviceChain::from(engine.new_input(1))
        .into(MoorerReverb::new(rooms::HALL, 1.0, -3.0, 0.5, 1))
        .into(engine.new_output(1));

    println!("Playing...");
    println!("Press enter to quit.");
    tick_until_enter(&mut chain);
    println!("Done!");
}
