import numpy as np

def fitEnvelope(x, attackTau=1.0, releaseTau=100.0):
    # Compute exponential multipliers from time constants
    # Convert from milliseconds to seconds
    attackAlpha = np.exp(-1.0/(44100.0*attackTau/1000.0))
    releaseAlpha = np.exp(-1.0/(44100.0*releaseTau/1000.0))

    # Perform leaky integration
    envelope = [0]*len(x)
    for i in range(1, len(x)):
        if np.abs(x[i]) > envelope[i-1]:
            alpha = attackAlpha
        else:
            alpha = releaseAlpha
        envelope[i] = alpha*envelope[i-1] + (1-alpha)*np.abs(x[i])

    return np.sqrt(envelope)
