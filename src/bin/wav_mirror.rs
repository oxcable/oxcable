//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;
extern crate portaudio;

#[cfg(not(test))]
fn main() {
    use oxcable::core::AudioDevice;
    use oxcable::io::wav;
    println!("Mirroring microphone input to speaker...");

    let mut inwav = wav::WavReader::new("wav/song.wav");
    let mut outwav = wav::WavWriter::new("wav/test_wav.wav", 2);
    outwav.inputs.set_channel(0, inwav.outputs.get_channel(0));
    outwav.inputs.set_channel(1, inwav.outputs.get_channel(1));

    let mut t: oxcable::core::Time = 1;
    while !inwav.is_done() {
        inwav.tick(t);
        outwav.tick(t);
        t += 1;
    }
    outwav.update_data_size()
}
