import math
if __name__ == "__main__":
    res = 0
    chains = []
    for x in range(1, 50):
        res += math.floor(1.3 ** x)
        chains.append(res)
    print(",".join(map(str, chains)))
