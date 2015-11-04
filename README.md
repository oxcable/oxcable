oxcable
=======

[Documentation](http://thenyeguy.github.io/oxcable/doc/oxcable/index.html)

[Crates.io](https://crates.io/crates/oxcable)

A signal processing framework for making music with Rust.

This is a personal project of mine, with two goals:
 1. To learn Rust.
 2. To work on design and implementation of real time audio systems.

I am still fleshing out the core libraries, so interfaces are still unstable.

Projects using oxcable
----------------------

* [oxcable-subtractive-synth](https://github.com/thenyeguy/oxcable-subtractive-synth),
  a MIDI subtractive synthesizer.

If you have a project using `oxcable`, I'd love to hear about it. Send me
a message and I can include it in this list.

Examples
--------

The following example will play back your computer's microphone input to the
speakers, with an echo effect:

```rust
use oxcable::delay::Delay;
use oxcable::chain::{DeviceChain, Tick};
use oxcable::io::audio::AudioEngine;

let engine = AudioEngine::with_buffer_size(256).unwrap();
let mut chain = DeviceChain::from(
    engine.default_input(1).unwrap()
).into(
    Delay::new(1.0, 0.5, 0.5, 1)
).into(
    engine.default_output(1).unwrap()
);
chain.tick_forever();
```

For more simple examples, the [`src/bin`](src/bin) directory contains many
sample test scripts.

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
