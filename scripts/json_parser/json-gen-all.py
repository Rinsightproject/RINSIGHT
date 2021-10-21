import json
import os
import sys

def check_scan_dir(scan_dir):
    return os.path.exists(scan_dir) and os.path.isdir(scan_dir)


def get_project_root():
    cwd_list = os.getcwd().split('/')
    need_path_list = []
    for item in cwd_list:
        need_path_list.append(item)
        if item == 'rust-up-to-you':
            break
    return '/'.join(need_path_list)


def gen_f_type_json(origin_f_path, output_f_path):
    print(origin_f_path)
    f_type_dict = {}
    with open(origin_f_path, 'r', encoding='utf8') as fp:
        data = json.load(fp)
        for entry in data:
            f_type = entry['f_type']
            if f_type in f_type_dict.keys():
                f_type_dict[f_type]['count'] += entry['count']
                f_type_dict[f_type]['percent'] += entry['percent']
            else:
                f_type_dict[f_type] = {'count': entry['count'], 'percent': entry['percent']}
    f_type_list = []
    for key in f_type_dict.keys():
        f_type_list.append({"f_type": key, "count": f_type_dict[key]['count'], "percent": f_type_dict[key]['percent']})
    print(json.dumps(f_type_list))
    with open(output_f_path, 'w') as fp:
        json.dump(f_type_list, fp)


if __name__ == '__main__':

    project_root = get_project_root()

    scan_dir = project_root + "/../downloads"

    output_path = project_root + '/data/json'
    if not os.path.exists(output_path):
        os.mkdir(output_path)
    print("output path:", output_path)

    if not check_scan_dir(scan_dir):
        print("scan dir invalid")
        exit(2)

    for cate in os.listdir(scan_dir):
        cate_dir = os.path.join(scan_dir, cate)
        if os.path.isdir(cate_dir):
            output_file = output_path + "/" + cate.replace("_data", "") + ".json"
            os.system("cargo run --release " + cate_dir + " -j > " + output_file)
            # arrange json by f_type
            if not os.path.exists(output_file):
                continue
            gen_f_type_json(output_file, output_path + '/' + cate.replace("_data", "") + "_f_type.json")

    total_file = output_path + "/total.json"
    os.system("cargo run --release " + scan_dir + " -j > " + total_file)
    gen_f_type_json(total_file, output_path + '/' + "total_f_type.json")
