//! Reads in MIDI events and uses them to trigger an ADSR

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::adsr;
    use oxcable::core::init;
    use oxcable::core::types::{Device, MidiEventType};
    use oxcable::io::audio::AudioOut;
    use oxcable::io::midi::MidiIn;
    use oxcable::oscillator;
    use oxcable::oscillator::Oscillator;

    println!("Initializing signal chain..."); 
    assert!(init::initialize().is_ok());

    let mut midi = MidiIn::new();
    let mut osc = Oscillator::new(oscillator::Sine, 440.0);
    let mut adsr = adsr::Adsr::default(1);
    let mut spk = AudioOut::new(1);
    adsr.inputs.set_channel(0, osc.output.get_channel());
    spk.inputs.set_channel(0, adsr.outputs.get_channel(0));

    let mut adsr_msg = adsr.messages.get_sender();

    println!("Playing...");
    let mut t = 0;
    loop {
        midi.tick(t);
        match midi.output.get(t) {
            Some(ref events) if events.len() > 0 => {
                for event in events.iter() {
                    match event.get_type() {
                        MidiEventType::NoteOn => 
                            adsr_msg.send(adsr::AdsrMessage::NoteDown),
                        MidiEventType::NoteOff => 
                            adsr_msg.send(adsr::AdsrMessage::NoteUp),
                        _ => ()
                    }
                }
            }
            _ => ()
        }
        osc.tick(t);
        adsr.tick(t);
        spk.tick(t);
        t += 1;
    }

    // midi.stop();
    // assert!(init::terminate().is_ok());
}
