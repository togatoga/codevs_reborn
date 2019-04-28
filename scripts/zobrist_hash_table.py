import random
import uuid
if __name__ == "__main__":
    y = 19
    x = 5
    num = 2 ** 8
    random.seed(114514)
    # 64 bit
    yxs = []
    for i in range(y):
        xs = []
        for j in range(x):
            hashs = []
            for k in range(num):
                hashs.append(random.randint(1, 2 ** 64))
            line = f'[{",".join(map(str, hashs))}]'
            xs.append(line)
        line = f'[{",".join(map(str, xs))}]'
        yxs.append(line)
    line = f'[{",".join(map(str, yxs))}]'
    print(line)
