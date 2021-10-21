import requests
import sys
from bs4 import BeautifulSoup
import os
import pandas as pd


def crawl(url):
    r = requests.get(url)
    soup = BeautifulSoup(r.content, "html.parser")
    stars = soup.find(name="a", attrs={"class": "social-count js-social-count"})
    stars_text = stars.text.strip()
    if stars_text.endswith("k"):
        stars_text = stars_text.strip("k")
        stars_text = float(stars_text) * 1000
    stars_count = int(stars_text)
    languages = soup.find_all(name="a", attrs={"class": "d-inline-flex flex-items-center flex-nowrap Link--secondary "
                                                        "no-underline text-small mr-3"})
    for language in languages:
        lan, percent = language.text.strip().split()
        if lan == "Rust":
            return stars_count, float(percent.strip("%"))
    return -1, -1


def get_data(param, data):
    avg_data = round(sum(data) / len(data), 1)
    min_data = min(data)
    max_data = max(data)
    return avg_data, min_data, max_data


def should_ignore_line(line):
    # EOF
    if line == "":
        return False
    line = line.strip()
    return line == "" or line.startswith("#")


def next_valid_line(f):
    while 1:
        line = f.readline()
        if not should_ignore_line(line):
            return line


def get_lines_and_files(proj_file_path):
    file_count = 0
    line_count = 0
    with open(proj_file_path) as data_file:
        for line in data_file.readlines():
            if "Total RS Files" in line:
                file_count = int(line.strip().split(':')[1])
            if "Total  Lines" in line:
                line_count = int(line.strip().split(':')[1])
    return line_count, file_count


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python3 raw_data_parser.py config_path")
        exit(1)
    cwd_list = os.getcwd().split('/')
    need_path_list = []
    for item in cwd_list:
        need_path_list.append(item)
        if item == 'rust-up-to-you':
            break
    data_path = '/'.join(need_path_list) + '/data'
    print("root data path:", data_path)
    all_proj_count = 0
    all_proj_stars = 0
    all_proj_rs_files = 0
    all_proj_rs_lines = 0
    domains = []
    num_of_projs = []
    KLOC_avg = []
    KLOC_min = []
    KLOC_max = []
    Files_avg = []
    Files_min = []
    Files_max = []
    Ratio_avg = []
    Ratio_min = []
    Ratio_max = []
    STARS_avg = []
    STARS_min = []
    STARS_max = []
    with open(sys.argv[1]) as f:
        while 1:
            category = next_valid_line(f)
            if category == "":
                break
            category = category.strip()
            domains.append(category)
            count = int(next_valid_line(f))
            num_of_projs.append(count)
            total_stars = []
            total_rust_percentage = []
            total_rs_lines = []
            total_rs_files = []
            for i in range(count):
                url = next_valid_line(f).strip()
                if url.endswith(".git"):# 用来扫描生成所有json，生成位置位于/data/json
# 可接受一个参数，是下载的目录路径，如../downloads
# 也可不带参数，默认在项目根目录上一级的downloads里面
# 可在项目下任意路径执行该脚本
                stars, rust_percentage = crawl(url)
                if stars == -1 and rust_percentage == -1:
                    print("errors with {}".format(url))
                    print("skipped")
                    continue
                proj_name = url.split('/')[-1]
                proj_data_path = data_path + '/' + category + '/' + proj_name
                # print("reading ", proj_data_path)
                lines, files = get_lines_and_files(proj_data_path)
                total_stars.append(stars)
                total_rust_percentage.append(rust_percentage)
                total_rs_lines.append(lines)
                total_rs_files.append(files)
            print("data for ", category)
            print("proj count", count)
            a, b, c = get_data("stars", total_stars)
            STARS_avg.append(a)
            STARS_min.append(b)
            STARS_max.append(c)
            a, b, c = get_data("rust percentage", total_rust_percentage)
            Ratio_avg.append(a)# 用来扫描生成所有json，生成位置位于/data/json
# 可接受一个参数，是下载的目录路径，如../downloads
# 也可不带参数，默认在项目根目录上一级的downloads里面
# 可在项目下任意路径执行该脚本
            Files_min.append(b)
            Files_max.append(c)
            a, b, c = get_data("rust code lines", total_rs_lines)
            KLOC_avg.append(a)
            KLOC_min.append(b)
            KLOC_max.append(c)
            all_proj_count += count
            all_proj_stars += sum(total_stars)
            all_proj_rs_files += sum(total_rs_files)
            all_proj_rs_lines += sum(total_rs_lines)
            print("")
    pd.set_option('display.max_colwidth',None)
    df = pd.DataFrame({'Domain': domains, 'Nums of Projects': num_of_projs, 'lines avg': KLOC_avg,
                       'lines min': KLOC_min, 'lines max': KLOC_max, 'files avg': Files_avg,
                       'files min': Files_min, 'files max': Files_max, 'Ratio avg': Ratio_avg, 'Ratio min': Ratio_min,
                       'Ratio max': Ratio_max, 'Stars avg': STARS_avg, 'Stars min': STARS_min, 'Stars max': STARS_max})
    print(df)

    print(df.to_latex)
