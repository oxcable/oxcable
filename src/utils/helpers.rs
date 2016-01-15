//! A collection of small utility functions.

use num::traits::Float;

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


#[cfg(test)]
mod tests {
    use testing::{flt_eq, flt_eq_eps};

    /// Tests some basic decibel values.
    #[test]
    fn test_decibel_to_ratio() {
        use super::decibel_to_ratio;
        assert!(flt_eq(decibel_to_ratio(-10.0), 0.1));
        assert!(flt_eq(decibel_to_ratio(-3.0), 0.501187234));
        assert!(flt_eq(decibel_to_ratio(0.0), 1.0));
        assert!(flt_eq(decibel_to_ratio(3.0), 1.995262315));
        assert!(flt_eq(decibel_to_ratio(10.0), 10.0));
    }

    /// Tests some basic ratio values.
    #[test]
    fn test_ratio_to_decibel() {
        use super::ratio_to_decibel;
        assert!(flt_eq(ratio_to_decibel(0.1), -10.0));
        assert!(flt_eq(ratio_to_decibel(0.501187234), -3.0));
        assert!(flt_eq(ratio_to_decibel(1.0), 0.0));
        assert!(flt_eq(ratio_to_decibel(1.995262315), 3.0));
        assert!(flt_eq(ratio_to_decibel(10.0), 10.0));
    }

    /// Tests some MIDI notes
    #[test]
    fn test_midi_note_to_freq() {
        use super::midi_note_to_freq;
        let epsilon = 1e-2f32; // use a smaller epsilon for imprecise freqs
        assert!(flt_eq_eps(midi_note_to_freq(21),    27.50, epsilon)); // A0
        assert!(flt_eq_eps(midi_note_to_freq(60),   261.63, epsilon)); // C4
        assert!(flt_eq_eps(midi_note_to_freq(69),   440.00, epsilon)); // A4
        assert!(flt_eq_eps(midi_note_to_freq(108), 4186.00, epsilon)); // C8
    }
}
