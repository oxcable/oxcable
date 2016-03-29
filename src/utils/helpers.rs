//! A collection of small utility functions.

use num::traits::Float;

use types::Sample;

/// Converts a decibel ratio to an amplitude multiplier.
pub fn decibel_to_ratio(db: f32) -> f32 {
    10.0.powf(db/10.0)
}

/// Converts an amplitude multiplier to decibels.
pub fn ratio_to_decibel(ratio: f32) -> f32 {
    10.0*ratio.log10()
}

/// Converts a MIDI note number to frequency in Hz.
pub fn midi_note_to_freq(note: u8) -> f32 {
    440.0*2.0.powf((note as f32 - 69.0) / 12.0)
}

/// Converts an i16 to a Sample type.
pub fn i16_to_sample(n: i16) -> Sample {
    (n as f32) / 32768.0
}

/// Converts a Sample to an i16.
///
/// If the Sample is outside the range [-1.0, 1.0), then it is clipped.
pub fn sample_to_16(s: Sample) -> i16 {
    let mut clipped = s;
    if clipped > 0.9999695 { clipped = 0.9999695; }
    if clipped < -1.0 { clipped = 1.0; }
    (clipped*32768.0) as i16
}


#[cfg(test)]
mod tests {
    use testing::{flt_eq, flt_eq_eps};

    #[test]
    fn test_decibel_to_ratio() {
        use super::decibel_to_ratio;
        assert!(flt_eq(decibel_to_ratio(-10.0), 0.1));
        assert!(flt_eq(decibel_to_ratio(-3.0), 0.501187234));
        assert!(flt_eq(decibel_to_ratio(0.0), 1.0));
        assert!(flt_eq(decibel_to_ratio(3.0), 1.995262315));
        assert!(flt_eq(decibel_to_ratio(10.0), 10.0));
    }

    #[test]
    fn test_ratio_to_decibel() {
        use super::ratio_to_decibel;
        assert!(flt_eq(ratio_to_decibel(0.1), -10.0));
        assert!(flt_eq(ratio_to_decibel(0.501187234), -3.0));
        assert!(flt_eq(ratio_to_decibel(1.0), 0.0));
        assert!(flt_eq(ratio_to_decibel(1.995262315), 3.0));
        assert!(flt_eq(ratio_to_decibel(10.0), 10.0));
    }

    #[test]
    fn test_midi_note_to_freq() {
        use super::midi_note_to_freq;
        let epsilon = 1e-2f32; // use a smaller epsilon for imprecise freqs
        assert!(flt_eq_eps(midi_note_to_freq(21),    27.50, epsilon)); // A0
        assert!(flt_eq_eps(midi_note_to_freq(60),   261.63, epsilon)); // C4
        assert!(flt_eq_eps(midi_note_to_freq(69),   440.00, epsilon)); // A4
        assert!(flt_eq_eps(midi_note_to_freq(108), 4186.00, epsilon)); // C8
    }

    #[test]
    fn test_i16_to_sample() {
        use super::i16_to_sample;
        assert_eq!(i16_to_sample(-32768), -1.0);
        assert_eq!(i16_to_sample(-16384), -0.5);
        assert_eq!(i16_to_sample(0), 0.0);
        assert_eq!(i16_to_sample(16384), 0.5);
        assert_eq!(i16_to_sample(32767), 1.0 - 1.0/32768.0);
    }

    #[test]
    fn test_sample_to_i16() {
        use super::sample_to_16;
        assert_eq!(sample_to_16(-1.0), -32768);
        assert_eq!(sample_to_16(-0.5), -16384);
        assert_eq!(sample_to_16(0.0), 0);
        assert_eq!(sample_to_16(0.5), 16384);
        assert_eq!(sample_to_16(1.0), 32767);

        // Verify clipping
        assert_eq!(sample_to_16(-2.0), -32768);
        assert_eq!(sample_to_16(2.0), 32767);
    }
}
