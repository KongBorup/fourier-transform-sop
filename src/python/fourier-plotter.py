import numpy as np
import matplotlib.pyplot as plt
from scipy import fftpack
import csv

signal = {
    'x': [],
    'y': [],
}

result = {
    'x': [],
    'y': [],
}

path = '/home/adrian/coding/projects/fourier-sop/data/plotdata.csv'

with open(path) as csv_file:
    csv_reader = csv.DictReader(csv_file)

    for row in csv_reader:
        if row['out_y'] == None:
            continue

        signal['x'].append(float(row['in_x']))
        signal['y'].append(float(row['in_y']))
        result['x'].append(float(row['out_x']))
        result['y'].append(float(row['out_y']))

n = len(result['x'])
t = signal['x']
x = signal['y']
f = result['x'][0:(n // 2)]
X = result['y'][0:(n // 2)]

plt.subplot(2, 1, 1)
plt.plot(t, x)
plt.xlabel('Time [s]')
plt.ylabel('Signal amplitude')

plt.subplot(2, 1, 2)
plt.plot(f, X)
plt.xlim(0, 10)
plt.ylim(0, 1)
plt.xlabel('Frequency [Hz]')
plt.ylabel('Intensity')

plt.show()