import json
import sys

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python3 raw_data_parser.py json_data_path n")
        exit(1)
    json_data_path = sys.argv[1]
    n = int(sys.argv[2])
    with open(json_data_path, 'r') as f:
        all_data = json.loads(f.read())['features']
        # print(all_data)
        sub = all_data[0:n]
        print(json.dumps(sub))
