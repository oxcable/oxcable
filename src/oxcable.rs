//! A signal processing framework for making music with rust.
//!
//! `oxcable` is designed with simple but powerful abstractions, with minimal
//! performance loss, while functioning in real-time.
//!
//! `oxcable` seeks to provide the basic building blocks necessary to build
//! digital instruments and effects, and provide the tools to chain these
//! devices together quickly and easily.

#![crate_name = "oxcable"]

extern crate byteorder;
extern crate num;
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
pub mod types;
pub mod utils;
pub mod voice_array;
