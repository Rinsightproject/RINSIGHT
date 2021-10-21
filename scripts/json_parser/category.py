import json
import sys
import os

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

    n = 5
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
    read_json = project_root+'/data/json/total.json'

    f = open(read_json, 'r')
    pre_path = project_root+'/data/json/'
    tail_path = '.json'
    json_files = ['cryptocurrencies', 'database', 'gaming', 'image_processing', 'security_tools', 'system_tool', 'web', 'os']
    all_data = json.loads(f.read())
    top = all_data[0:n]
    category = []
    for feature in top:
        feature_name = feature['name']
        m = {'feature_name': feature_name, 'feature_type': feature['f_type']}
        l = []
        for json_file in json_files:
            with open(pre_path + json_file + tail_path, 'r') as ff:
                data = json.loads(ff.read())
                for d in data:
                    if d['name'] == feature_name:
                        l.append({'category': json_file, 'percent': d['percent']})
                        break
        m['categories'] = l
        category.append(m)
    print(json.dumps(category))
    f.close()
