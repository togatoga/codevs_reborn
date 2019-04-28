import random
import uuid
if __name__ == "__main__":
    num = 2 ** 64
    random.seed(810)
    # 64 bit
    scores = []
    for i in range(2000):
        scores.append(random.randint(1, num))
    line = f'[{",".join(map(str, scores))}]'
    print(line)
