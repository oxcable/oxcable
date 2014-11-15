//! Provides parameter sets to simulate different reverberant environments.

#![unstable]

use std::vec::Vec;

use core::Time;


/// A room defines three sets of numbers.
///
/// The tapped delays and tapped gains specify the start of the reverb, and must
/// be the same length. They represent the initial multipaths, and are used to
/// sculpt the echoes of the room.
///
/// The comb delays are used to fill out the reverberations and create a steady
/// decay; they sculpt the character of the room.
pub struct Room {
    pub tapped_delays: Vec<Time>,
    pub tapped_gains: Vec<f32>,
    pub comb_delays: Vec<Time>
}

// A simulation of a concert hall, originally designed by James Moorer for his
// paper, "About This Reverberation Business".
pub fn hall() -> Room {
    Room {
        // For these parameters, see pg. 24 from Moorer paper
        tapped_delays: vec![190, 948, 992, 1182, 1191, 1314, 2020, 2523, 2589,
            2624, 2699, 3118, 3122, 3202, 3268, 3321, 3515],
        tapped_gains: vec![0.841, 0.504, 0.491, 0.379, 0.380, 0.346, 0.289, 0.272,
            192.0, 0.193, 0.217, 0.181, 0.180, 0.181, 0.176, 0.142, 0.167, 0.134],
        // For these parameters, see pg. 18 from Moorer
        comb_delays: vec![2205, 2470, 2690, 2999, 3175, 3440],
    }
}
