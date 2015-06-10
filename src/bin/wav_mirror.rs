//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::wav;

    println!("Initializing signal chain...");
    let inwav = wav::WavReader::new("wav/song.wav");
    let samples = inwav.get_num_samples();
    let outwav = wav::WavWriter::new("wav/test_wav.wav", 2);
    let mut chain = DeviceChain::from(inwav).into(outwav);

    println!("Mirroring wav/song.wav input to wav/test_wav.wav...");
    for _ in 0..samples {
        chain.tick();
    }
}
