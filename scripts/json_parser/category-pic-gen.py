import os
import sys


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

    json_output_path = project_root + '/data/json/category.json'
    pic_output_path = project_root + '/data/pic/category.jpg'

    top_n = 15
    if len(sys.argv) > 1:
        top_n = sys.argv[1]

    os.system("python3 "+project_root+"/scripts/json_parser/category.py "+str(top_n) \
        +"  > "+json_output_path)

    
    os.system("python3 "+project_root+"/scripts/draw_bar/draw_category_bar.py "+\
        json_output_path+" "+pic_output_path)