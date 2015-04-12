//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::init;
    use oxcable::components::DeviceManager;
    use oxcable::types::Device;
    use oxcable::io::audio::{AudioIn, AudioOut};

    println!("Initializing signal chain...");
    assert!(init::initialize().is_ok());

    let mut mic = AudioIn::new(1);
    let mut spk = AudioOut::new(1);
    spk.inputs.set_channel(0, mic.outputs.get_channel(0));

    let mut manager = DeviceManager::new();
    manager.add_device(&mut mic);
    manager.add_device(&mut spk);

    println!("Mirroring microphone input to speaker...");
    loop {
        manager.tick();
    }
}
