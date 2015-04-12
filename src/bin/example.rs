//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::rc::Rc;
    use oxcable::oscillator;
    use oxcable::components::DeviceManager;
    use oxcable::delay::Delay;
    use oxcable::io::audio::{AudioEngine, AudioIn, AudioOut};
    use oxcable::mixers::Gain;
    use oxcable::oscillator::Oscillator;
    use oxcable::types::Device;

    println!("Setting up signal chain...");
    let engine = Rc::new(AudioEngine::open().unwrap());

    let mut mic = AudioIn::new(engine.clone(), 1);
    let mut del = Delay::new(0.5, 0.5, 0.5, 1);
    del.inputs.set_channel(0, mic.outputs.get_channel(0));

    let mut osc = Oscillator::new(oscillator::Sine, 440.0);
    let mut gain = Gain::new(-12.0, 1);
    gain.inputs.set_channel(0, osc.output.get_channel());

    let mut spk = AudioOut::new(engine.clone(), 2);
    spk.inputs.set_channel(0, del.outputs.get_channel(0));
    spk.inputs.set_channel(1, gain.outputs.get_channel(0));

    let mut manager = DeviceManager::new();
    manager.add_device(&mut mic);
    manager.add_device(&mut del);
    manager.add_device(&mut osc);
    manager.add_device(&mut gain);
    manager.add_device(&mut spk);

    println!("Playing...");
    loop {
        manager.tick();
    }
}
