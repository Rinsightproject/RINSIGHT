# -*- coding: utf-8 -*-
import json

if __name__ == '__main__':
    test_dict = {}

    labels = ['USA', 'China', 'India', 'Japan', 'Germany', 'Russia', 'Brazil', 'UK', 'France', 'Italy']
    quants = [15094025.0, 11299967.0, 4457784.0, 4440376.0, 3099080.0, 2383402.0, 2293954.0, 2260803.0, 2217900.0,
              1846950.0]

    entries = []
    for i in range(len(labels)):
        entries.append({
            "data": quants[i],
            "label": labels[i]

        })
    test_dict["entries"] = entries
    test_dict["xlabel"] = 'Country'
    test_dict["ylabel"] = 'GDP (Billion US dollar)'
    test_dict["title"] = 'Top 10 GDP Countries'
with open("test.json", 'w') as f:
    f.write(json.dumps(test_dict))
