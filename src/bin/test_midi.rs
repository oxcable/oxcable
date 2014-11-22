//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::core::init;
    use oxcable::core::types::Device;
    use oxcable::io::midi::MidiIn;

    println!("Initializing signal chain..."); 
    assert!(init::initialize().is_ok());

    let mut midi = MidiIn::new();

    println!("Running loop");
    let mut t = 0;
    loop {
        midi.tick(t);
        match midi.output.get(t) {
            Some(ref events) if events.len() > 0 => println!("{}", events),
            _ => ()
        }
        t += 1;
    }

    // midi.stop();
    // assert!(init::terminate().is_ok());
}
