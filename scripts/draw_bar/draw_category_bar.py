import json

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import os
import sys
wanted_feature = []


def get_project_root():
    cwd_list = os.getcwd().split('/')
    need_path_list = []
    for item in cwd_list:
        need_path_list.append(item)
        if item == 'rust-up-to-you':
            break
    return '/'.join(need_path_list)


if __name__ == '__main__':

    project_root = get_project_root()
    
    json_path = project_root + '/data/json/category.json'
    save_path = project_root + '/data/pic/category.jpg'
    argc = len(sys.argv)
    if argc > 1:
        json_path = sys.argv[1]
    if argc > 2:
        save_path = sys.argv[2]
    
    
    print("opening json file ", json_path)
    index = []
    columns = []
    data = []
    columns_fill = False
    with open(json_path) as f:
        load_json = json.load(f)
        for entry in load_json:
            if not columns_fill:
                for item in entry['categories']:
                    columns.append(item['category'])
                columns_fill = True
            index.append(entry['feature_name'])
            tmp_data = []
            for item in entry['categories']:
                tmp_data.append(item['percent'])
            data.append(tmp_data)

    for i in range(0,len(index)):
        index[i] = index[i].replace(' ','\n')
    df = pd.DataFrame(data, columns=columns,index=index)

    ax = df.plot(kind='bar',rot=0,fontsize=8,figsize=(10,5))


    plt.savefig(save_path, bbox_inches='tight', pad_inches=0.1)
    plt.show()
