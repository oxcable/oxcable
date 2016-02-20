oxcable
=======

[![Build Status](https://travis-ci.org/oxcable/oxcable.svg?branch=master)](https://travis-ci.org/oxcable/oxcable)
[![Crates.io](https://img.shields.io/crates/v/oxcable.svg)](https://crates.io/crates/oxcable)
![License](https://img.shields.io/crates/l/oxcable.svg)

[Documentation](http://oxcable.github.io/oxcable/doc/oxcable/index.html)

A signal processing framework for making music with Rust.

This framework aims to provide the basic tools for music processing. It defines
many useful types, utilities, signal processing elements for building music
processing pipelines.

For a basic set of predefined filters, see
[oxcable-basic-devices](https://github.com/oxcable/basic-devices).

Examples
--------

The following example will play back your computer's microphone input to the
speakers, with an low-pass effect:

```rust
use oxcable::chain::{DeviceChain, Tick};
use oxcable::filters::first_order::{Filter, LowPass};
use oxcable::io::audio::AudioEngine;

let engine = AudioEngine::with_buffer_size(256).unwrap();
let mut chain = DeviceChain::from(
    engine.default_input(1).unwrap()
).into(
    Filter::new(LowPass(8000f32), 1)
).into(
    engine.default_output(1).unwrap()
);
chain.tick_forever();
```

For more simple examples, the [`src/bin`](src/bin) directory contains many
sample test scripts.

Projects using oxcable
----------------------

* [oxcable-subtractive-synth](https://github.com/oxcable/oxcable-subtractive-synth),
  a MIDI subtractive synthesizer.

If you have a project using `oxcable`, I'd love to hear about it. Send me
a message and I can include it in this list.

Installing
----------

Currently, oxcable requires PortAudio and PortMIDI be installed on your machine.

On Mac, these are available on Homebrew. To install, run:

    brew install portaudio
    brew install portmidi

Scripts
-------

The scripts directory contains assorted scripts used both to experiment with new
features, and to test the output of Rust library code. These scripts are written
in Python, and leverage the `numpy`, `scipy` and `matplotlib` libraries for
rapid prototyping purposes.

License
-------

Licensed under either of

 * Apache License, Version 2.0 ([license-apache.txt](license-apache.txt) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([license-mit.txt](license-mit.txt) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
