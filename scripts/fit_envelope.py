import numpy as np

def fit_envelope(x, attack_tau=1.0, release_tau=100.0):
    # Compute exponential multipliers from time constants
    # Convert from milliseconds to seconds
    attack_alpha = np.exp(-1.0/(44100.0*attack_tau/1000.0))
    release_alpha = np.exp(-1.0/(44100.0*release_tau/1000.0))

    # Perform leaky integration
    envelope = [0]*len(x)
    for i in range(1, len(x)):
        if np.abs(x[i]) > envelope[i-1]:
            alpha = attack_alpha
        else:
            alpha = release_alpha
        envelope[i] = alpha*envelope[i-1] + (1-alpha)*np.abs(x[i])

    return np.sqrt(envelope)
