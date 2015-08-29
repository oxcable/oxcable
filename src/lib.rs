//! A signal processing framework for making music with rust.
//!
//! `oxcable` is designed with simple but powerful abstractions, with minimal
//! performance loss, while functioning in real-time.
//!
//! `oxcable` seeks to provide the basic building blocks necessary to build
//! digital instruments and effects, and provide the tools to chain these
//! devices together quickly and easily.
//!
//! # Getting started
//!
//! Let's start with a simple example. This script will generate a sine wave at
//! 440Hz, and play it on your computer's speakers or headphones in an infinite
//! loop:
//!
//! ```
//! extern crate oxcable;
//! use oxcable::chain::{DeviceChain, Tick};
//! use oxcable::io::audio::AudioEngine;
//! use oxcable::oscillator::{Oscillator, Sine};
//!
//! fn main() {
//!     let engine = AudioEngine::with_buffer_size(256).unwrap();
//!     let mut chain = DeviceChain::from(
//!         Oscillator::new(Sine).freq(440.0)
//!     ).into(
//!         engine.default_output(1).unwrap()
//!     );
//! # }
//! # // wrap tick_forever() in a function that gets compiled by not run by
//! # //doctest
//! # fn dummy(chain: &mut DeviceChain) {
//!     chain.tick_forever();
//! }
//! ```
//!
//! # Devices
//!
//! Defining an audio device can be done by implenting the
//! [`AudioDevice` trait](types/trait.AudioDevice.html).
//!
//! Let's define a very simple audio device: one that simply passes its input
//! straight to the output. This can be done in just a few lines:
//!
//! ```
//! use oxcable::types::{AudioDevice, Sample, Time};
//!
//! struct IdentityFilter;
//! impl AudioDevice for IdentityFilter {
//!     fn num_inputs(&self) -> usize { 1 }
//!     fn num_outputs(&self) -> usize { 1 }
//!     fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
//!         outputs[0] = inputs[0];
//!     }
//! }
//! ```
//!
//! Once we've defined a device, it is simple to use the device in isolation.
//! Say I want to test my filter. I can manually call tick a few times to
//! generate outputs:
//!
//! ```
//! # use oxcable::types::{AudioDevice, Sample, Time};
//! # struct IdentityFilter;
//! # impl AudioDevice for IdentityFilter {
//! #     fn num_inputs(&self) -> usize { 1 }
//! #     fn num_outputs(&self) -> usize { 1 }
//! #     fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
//! #         outputs[0] = inputs[0];
//! #     }
//! # }
//! #
//! let mut input = [0.0];
//! let mut output = [0.0];
//! let mut filter = IdentityFilter;
//!
//! for i in 0..8 {
//!     input[0] = i as f32;
//!     filter.tick(i, &input, &mut output);
//!     assert_eq!(i as f32, output[0]);
//! }
//! ```
//!
//! By adhering to the `AudioDevice` trait, however, this new device will drop
//! straight into generic containers like [`DeviceChain`](chain/index.html) or
//! [`DeviceGraph`](graph/index.html), and into wrappers such as the `Buffered`
//! wrapper:
//!
//! ```
//! # use oxcable::types::{AudioDevice, Sample, Time};
//! # struct IdentityFilter;
//! # impl AudioDevice for IdentityFilter {
//! #     fn num_inputs(&self) -> usize { 1 }
//! #     fn num_outputs(&self) -> usize { 1 }
//! #     fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
//! #         outputs[0] = inputs[0];
//! #     }
//! # }
//! #
//! use oxcable::wrappers::Buffered;
//! let mut filter = Buffered::from(IdentityFilter);
//!
//! for i in 0..8 {
//!     filter.inputs[0] = i as f32;
//!     filter.tick(i);
//!     assert_eq!(i as f32, filter.outputs[0]);
//! }
//! ```
//!
//! `oxcable` defines many simple devices itself. A list of these devices may be
//! found under [the `AudioDevice`
//! documentation](types/trait.AudioDevice.html#implementors).

extern crate byteorder;
extern crate num;
extern crate portaudio;
extern crate portmidi;
extern crate rand;

pub mod adsr;
pub mod chain;
pub mod delay;
pub mod dynamics;
pub mod filters;
pub mod graph;
pub mod io;
pub mod mixers;
pub mod oscillator;
pub mod reverb;
pub mod tick;
pub mod tremolo;
pub mod types;
pub mod utils;
pub mod voice_array;
pub mod wrappers;
