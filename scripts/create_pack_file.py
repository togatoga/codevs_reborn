import os
import sys


def read_pack(f):
    result = ""
    for (idx, line) in enumerate(f):
        if idx >= 1500:
            break
        result += line
    return result


def main(args):
    root_dir = args[1]
    output_pack_dir = args[2]
    files = os.listdir(args[1])
    stdin_files = list(filter(lambda x: x.endswith("_stdin.txt"), files))
    packs = set()
    for stdin_file in stdin_files:
        stdin_file_path = os.path.join(root_dir, stdin_file)
        with open(stdin_file_path, "r") as f:
            packs.add(read_pack(f))

    for (idx, pack) in enumerate(packs):
        output_file_name = f'pack_{idx:0>4}.pack'
        output_file_path = os.path.join(output_pack_dir, output_file_name)
        with open(output_file_path, "w") as f:
            f.write(pack)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
