//! A collection of small utility functions.

use num::traits::Float;


/// Compares floating point numbers for equality.
///
/// The maximum difference is specified by `epsilon`; that is, `f1` and `f2` are
/// equal if they differ by at most epsilon.
pub fn flt_eq(f1: f32, f2: f32, epsilon: f32) -> bool {
    (f1 - f2).abs() < epsilon
}

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
    use super::flt_eq;
    static EPSILON: f32 = 1e-6;

    /// Tests some basic decibel values.
    #[test]
    fn test_decibel_to_ratio() {
        use super::decibel_to_ratio;
        assert!(flt_eq(decibel_to_ratio(-10.0), 0.1, EPSILON));
        assert!(flt_eq(decibel_to_ratio(-3.0), 0.501187234, EPSILON));
        assert!(flt_eq(decibel_to_ratio(0.0), 1.0, EPSILON));
        assert!(flt_eq(decibel_to_ratio(3.0), 1.995262315, EPSILON));
        assert!(flt_eq(decibel_to_ratio(10.0), 10.0, EPSILON));
    }

    /// Tests some basic ratio values.
    #[test]
    fn test_ratio_to_decibel() {
        use super::ratio_to_decibel;
        assert!(flt_eq(ratio_to_decibel(0.1), -10.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(0.501187234), -3.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(1.0), 0.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(1.995262315), 3.0, EPSILON));
        assert!(flt_eq(ratio_to_decibel(10.0), 10.0, EPSILON));
    }

    /// Tests some MIDI notes
    #[test]
    fn test_midi_note_to_freq() {
        use super::midi_note_to_freq;
        let epsilon = 1e-2f32; // use a smaller epsilon for imprecise freqs
        assert!(flt_eq(midi_note_to_freq(21),    27.50, epsilon)); // A0
        assert!(flt_eq(midi_note_to_freq(60),   261.63, epsilon)); // C4
        assert!(flt_eq(midi_note_to_freq(69),   440.00, epsilon)); // A4
        assert!(flt_eq(midi_note_to_freq(108), 4186.00, epsilon)); // C8
    }
}
