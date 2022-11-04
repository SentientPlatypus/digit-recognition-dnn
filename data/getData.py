import gzip
import pickle
import matplotlib.cm as cm
import matplotlib.pyplot as plt
import os

for f in os.listdir("W:\Code\Machine Learning\digit-recognition-dnn\data"):
    print(f)

name = 'mnist.gz'

with gzip.open(name, 'rb') as f:
    train_set, valid_set, test_set = pickle.load(f)

train_x, train_y = train_set

plt.imshow(train_x[0].reshape((28, 28)), cmap=cm.Greys_r)
plt.show()