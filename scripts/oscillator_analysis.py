#!/usr/bin/env python3

import matplotlib.pyplot as plot
import numpy as np
import scipy.io.wavfile as wav
import sys

basic_waveforms = ["sine", "white_noise", "pulse_train"]
blep_waveforms = ["saw", "square", "tri"]
waveforms = basic_waveforms + blep_waveforms


def load_audio_data(waveform):
    if waveform in blep_waveforms:
        filenames = ["wav/test_{}_naive.wav".format(waveform),
                     "wav/test_{}_blep.wav".format(waveform)]
    else:
        filenames = ["wav/test_{}.wav".format(waveform)]

    xs = []
    for filename in filenames:
        _, x = wav.read(filename)
        xs.append(x.astype(float) / 32767.0)
    return filenames, xs


def plot_spectra(title, names, xs):
    fs = range
    colors = ['blue', 'red', 'green']
    for (name,x, color) in zip(names, xs, colors):
        Xmag = np.absolute(np.fft.fft(x, 5000))
        fs = np.linspace(0, 44100/2, 5000/2)
        X = 20*np.log10(Xmag/max(Xmag)) # convert to decibels
        plot.plot(fs, X[:len(X)/2], label=name, color=color)
    plot.legend()
    plot.title(title)
    plot.show()


def usage():
    print("Usage:", sys.argv[0], "<waveform type>")
    print()
    print("Supported waveforms:", ", ".join(waveforms))
    sys.exit(0)

if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] not in waveforms:
        usage()
    waveform = sys.argv[1]
    filenames, xs = load_audio_data(waveform)
    plot_spectra("Spectra for {}".format(waveform), filenames, xs)
