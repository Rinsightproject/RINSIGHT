import os
import sys
import shutil


del_list=[]

def walk_dir(dir_name):
    for f in os.listdir(dir_name):
        file_dir = os.path.join(dir_name,f)
        if os.path.isdir(file_dir):
            if f.__eq__("tests"):
                del_list.append(file_dir)
            else:
                walk_dir(file_dir)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 raw_data_parser.py config_path")
        exit(1)
    scan_dir = sys.argv[1]
    if not os.path.exists(scan_dir):
        print("dir not exists")
        exit(2)
    if os.path.isfile(scan_dir):
        print("this is a file path")
        exit(3)
    walk_dir(scan_dir)

    should_del = False
    if len(sys.argv) == 3 and sys.argv[2].__eq__("-d"):
        should_del = True

    for d in del_list:
        print(d)
        if should_del:
            shutil.rmtree(d)
