import random
if __name__ == "__main__":
    MAX_TURN = 500
    for i in range(500):
        x1 = random.randint(0, 9)
        x2 = random.randint(0, 9)
        x3 = random.randint(0, 9)
        x4 = random.randint(0, 9)
        print(x1, x2)
        print(x3, x4)
        print("END")
    print(0)  # turn
    # player
    print(180000)  # time
    print(0)  # dead block
    print(0)  # skill point
    print(0)  # current score
    MAX_WIDTH = 10
    MAX_HEIGHT = 16
    for i in range(MAX_HEIGHT):
        res = [0 for j in range(MAX_WIDTH)]
        print(" ".join(map(str, res)))
    print("END")
    # enemy
    print(180000)  # time
    print(0)  # dead block
    print(0)  # skill point
    print(0)  # current score
    MAX_WIDTH = 10
    MAX_HEIGHT = 16
    for i in range(MAX_HEIGHT):
        res = [0 for j in range(MAX_WIDTH)]
        print(" ".join(map(str, res)))
    print("END")
