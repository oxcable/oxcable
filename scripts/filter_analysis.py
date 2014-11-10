import matplotlib.pyplot as plot
import numpy as np
import scipy.io.wavfile as wav
import sys

filters = ["first_order_low_pass",  "first_order_high_pass",
           "first_order_low_shelf", "first_order_high_shelf"]


def load_audio_data(filter_type):
    filename = "../wav/test_{}.wav".format(filter_type)
    _, x = wav.read(filename)
    return (filename, x.astype(float) / 32767.0)


def plot_spectra(title, xs):
    Xmag = np.absolute(np.fft.fft(x, 5000))
    X = 20*np.log10(Xmag/max(Xmag)) # convert to decibels
    fs = np.fft.fftfreq(len(X), 1.0/44100) # get freq labels
    plot.semilogx(fs[:len(fs)/2], X[:len(X)/2])
    plot.title(title)
    plot.show()


def usage():
    print("Usage:", sys.argv[0], "<filter type>")
    print()
    print("Supported filters:", ", ".join(filters))
    sys.exit(0)

if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] not in filters:
        usage()
    filter_type = sys.argv[1]
    filename, x = load_audio_data(filter_type)
    plot_spectra("Spectra for {}".format(filename), x)
