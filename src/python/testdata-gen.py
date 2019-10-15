import numpy as np
import matplotlib.pyplot as plt
from scipy import fftpack
import csv

fs = 1024 # Sampling frequency [Hz]
f = 3 # Frequency of sound signal [Hz]
t_max = 4 # Duration of time [s]

t = np.arange(0, t_max, 0.5 / fs)
x1 = np.cos(f * t_max * np.pi * t)
x2 = np.cos(f * 0.843 * t_max * np.pi * t)
x = x1 + x2

X = fftpack.fft(x)

n = len(x)
k = np.arange(n)
T = n / fs
frq = k/T
# frq = frq[range(n//2)]

X = np.abs(X / n)
# X = X[range(n//2)]

with open('/home/adrian/coding/projects/fourier-sop/data/plotdata.csv', mode='w') as csv_file:
    fieldnames = ['in_x', 'in_y', 'out_x', 'out_y']
    csv_writer = csv.DictWriter(csv_file, fieldnames=fieldnames)

    csv_writer.writeheader()

    for i in range(len(x)):
        csv_writer.writerow({
            'in_x': t[i],
            'in_y': x[i],
            'out_x': frq[i],
            'out_y': X[i],
        })
quit()
plt.subplot(2, 1, 1)
plt.plot(t, x)
plt.xlabel('Time [s]')
plt.ylabel('Signal amplitude')

plt.subplot(2, 1, 2)
plt.plot(frq[range(n // 2)], X[range(n // 2)])
plt.xlim(0, 10)
plt.ylim(0, 1)
plt.xlabel('Frequency [Hz]')
plt.ylabel('Intensity')

# plt.show()