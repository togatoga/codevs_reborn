import sys
import os
from collections import defaultdict


def read_pack(f):
    packs = []
    pack = []
    for line in f:
        if line == "END\n":
            packs.append(pack)
            pack = []
            continue
        else:
            pack.append(map(int, line.rstrip().split()))
    return packs


def analyze_packs(packs):
    counter_number_block = defaultdict(int)
    counter_pack_type = defaultdict(int)
    for idx, pack in enumerate(packs):
        assert(len(pack) == 2)
        pack_count = 4
        for (x, y) in zip(pack[0], pack[1]):
            if x == 0:
                pack_count -= 1
            if y == 0:
                pack_count -= 1
            counter_number_block[x] += 1
            counter_number_block[y] += 1
            counter_pack_type[pack_count] += 1
    for i in range(1, 11):
        print(f'{i}:{counter_number_block[i]}')
    print(counter_number_block)
    print(counter_pack_type)


def main(args):
    pack_file = args[1]

    with open(pack_file, "r") as f:
        packs = read_pack(f)
        analyze_packs(packs)

    return os.EX_OK


if __name__ == "__main__":
    sys.exit(main(sys.argv))
