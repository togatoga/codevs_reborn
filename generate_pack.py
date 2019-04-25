import os
import sys


def main(args):
    print(args)
    print(os.listdir(args.[1]))


if __name__ == "__main__":
    sys.exit(main(sys.argv))
