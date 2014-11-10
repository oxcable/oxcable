OxilaryCable
=============

A simple audio processing and MIDI framework in Rust.

This is a personal project of mine, with two goals:
 1. To learn Rust.
 2. To work on design and implementation of real time audio systems.

I am still fleshing out the core libraries, so interfaces are very unstable.

Installing
----------

Currently. OxilaryCable requires PortAudio be installed on your machine. On Mac,
this is available on Homebrew. To install, run:

    brew install portaudio

Scripts
-------

The scripts directory contains assorted scripts used both to experiment with new
features, and to test the output of Rust library code. These scripts are written
in Python, and leverage the `numpy`, `scipy` and `matplotlib` libraries for
rapid prototyping purposes.

ClickTrack
----------

This project is a successor to another project of mine ClickTrack. ClickTrack is
an audio framework written in C++. Much of this project is a port of work
originally done in ClickTrack. To see more, visit 
[my ClickTrack repository](https://github.com/thenyeguy/ClickTrack).
