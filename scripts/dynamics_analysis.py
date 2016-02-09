#!/usr/bin/env python3

import matplotlib.pyplot as plot
import numpy as np
import scipy.io.wavfile as wav
import sys

from fit_envelope import fit_envelope


dynamics = ["compressor", "limiter", "noise_gate"]


def load_audio_data():
    filenames = []
    for dynamic in dynamics:
        filenames.append("wav/test_{}.wav".format(dynamic))
    filenames.append("wav/volume_up.wav");

    xs = []
    for filename in filenames:
        _, x = wav.read(filename)
        xs.append(x.astype(float) / 32767.0)
    return filenames, xs


def plot_signals(title, names, xs):
    fs = range
    for (name,x) in zip(names, xs):
        plot.plot(fit_envelope(x), label=name)
    plot.legend(loc="upper left")
    plot.title(title)
    plot.show()


if __name__ == "__main__":
    filenames, xs = load_audio_data()
    plot_signals("Dynamic processors", filenames, xs)
