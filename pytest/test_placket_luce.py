import pypermu
import numpy as np

pl = pypermu.distributions.placket_luce

w = np.random.rand(5)

print('weights:\n\t', w)
print('samples:\n', np.array(pl.sample_pl(w, 10)))
