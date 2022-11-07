import gzip
import pickle
import matplotlib.cm as cm
import matplotlib.pyplot as plt
import os
import json
import numpy as np

class NpEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        if isinstance(obj, np.floating):
            return float(obj)
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return super(NpEncoder, self).default(obj)


name = 'W:\Code\Machine Learning\digit-recognition-dnn\data\mnist.pkl.gz'
data_json_path = "data\data.json"



with gzip.open(name, 'rb') as f:
    u = pickle._Unpickler( f )
    u.encoding = 'latin1'
    train_set, valid_set, test_set = u.load()
train_x, train_y = train_set
count = 1


def getDictionary(path:str = data_json_path) ->dict:
    """Returns python dictionary from JSON file at path"""
    with open(path, "r", encoding="utf-8") as read:
        return json.load(read)

def updateDatabase(newDictionary:dict, path:str = data_json_path):
    with open(path, "w") as File:
        json.dump(newDictionary, fp=File, indent=4, cls=NpEncoder)


json_current = getDictionary()
for index in range(len(train_x)):
    example = train_x[index]
    y = train_y[index]
    vector:np.ndarray = example.reshape((1,28 * 28))[0].tolist()
    print(count)
    count+=1
    img_ob = {
        "vector":vector,
        "y":y
    }
    json_current["data"].append(img_ob)
    # json_object = json.dumps(img_ob, indent=4, cls=NpEncoder)
    # with open("W:\Code\Machine Learning\digit-recognition-dnn\data\data.json", "a") as outfile:
    #     outfile.write(json_object)
    # print(vector)
    # print(y)
    # plt.imshow(example.reshape((28, 28)), cmap=cm.Greys_r)
    # plt.show()
updateDatabase(json_current)
