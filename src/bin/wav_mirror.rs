//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::types::Device;
    use oxcable::io::wav;
    println!("Mirroring wav/song.wav input to wav/test_wav.wav...");

    let mut inwav = wav::WavReader::new("wav/song.wav");
    let mut outwav = wav::WavWriter::new("wav/test_wav.wav", 2);
    outwav.inputs.set_channel(0, inwav.outputs.get_channel(0));
    outwav.inputs.set_channel(1, inwav.outputs.get_channel(1));

    let mut t = 0;
    while !inwav.is_done() {
        inwav.tick(t);
        outwav.tick(t);
        t += 1;
    }
    outwav.update_data_size()
}
