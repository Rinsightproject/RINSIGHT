# -*- coding: utf-8 -*-
import json

import numpy as np
import matplotlib.pyplot as plt
import matplotlib as mpl
import sys


def draw_bar(load_json, path_name, draw_percent):
    label = []
    data = []
    for entry in load_json:
        label.append(entry["name"])
        if draw_percent:
            data.append(entry["percent"])
        else:
            data.append(entry['count'])
    width = 4
    label.reverse()
    data.reverse()
    ind = np.linspace(0.5, len(load_json) * 10, len(load_json))
    # make a square figure
    fig = plt.figure(figsize=(18,10))
    ax = fig.add_subplot(111)

    # Bar Plot
    b = ax.bar(x=0, bottom=label, width=data, height=0.9,color='green', orientation="horizontal", )

    for rect in b:
        w = rect.get_width()
        if draw_percent:
            ax.text(w, rect.get_y() + rect.get_height() / 2, ' %.2f%%' %
                    w, ha='left', va='center')
        else:
            ax.text(w, rect.get_y() + rect.get_height() / 2, ' %d' %
                    w, ha='left', va='center')
    # Set the ticks on x-axis
    # ax.set_xticks(ind)
    # ax.set_xticklabels(label)
    # labels
    if (draw_percent):
        ax.set_xlabel("percent")
    else:
        ax.set_xlabel("count")
    ax.set_ylabel("feature")
    # title

    ax.set_title(path_name)

    ax.grid(axis="x",ls="--")
    plt.grid(axis="x",ls="--")
    plt.savefig(path_name + ".jpg", bbox_inches = 'tight')

    plt.show()
    plt.close()


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 draw_bar.py json_data_path save_file_name")
        exit(1)
    draw_percent = True
    if len(sys.argv) > 3 and sys.argv[3] == '-a':
        draw_percent = False
    with open(sys.argv[1], 'r') as load_f:
        load_json = json.load(load_f)
        draw_bar(load_json, sys.argv[2], draw_percent)
