#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import random
import sys
import copy

AI_NAME = "SampleAI.py"
width = 10
height = 16
packSize = 2
summation = 20
maxTurn = 500
simulationHeight = height + packSize + 1
OBSTACLE_BLOCK = summation + 1
EMPTY_BLOCK = 0
packs = []


# 標準入力からパックを得ます
def inputPack():
    pack = [[int(i) for i in input().split()] for j in range(packSize)]
    input()  # END
    return pack


# パックを90度回転させます
def rotateOnce(pack):
    rotated = copy.deepcopy(pack)
    for i in range(packSize):
        for j in range(packSize):
            rotated[j][packSize - 1 - i] = pack[i][j]
    return rotated


# パックを指定した回数だけ90度回転させます
def rotate(pack, rotation):
    for _ in range(rotation):
        pack = rotateOnce(pack)
    return pack


# 標準エラー出力にパックの情報を出力します
def printPack(pack):
    print('\n'.join(map(lambda row: ' '.join(map(lambda block: '{:>2}'.format(block), row)), pack)), file=sys.stderr)
    sys.stdout.flush()


# 標準入力から盤面を得ます
def inputField():
    field = [[EMPTY_BLOCK] * width if j < simulationHeight - height else [int(i) for i in input().split()]
             for j in range(simulationHeight)]
    input()  # END
    return field


# def alternativeInputField():
#    field = []
#    for j in range(simulationHeight):
#        if j < simulationHeight - height:
#            row = []
#            for i in range(width):
#                row.append(EMPTY_BLOCK)
#            field.append(row)
#        else:
#            row = []
#            temp = input().split()
#            for i in range(width):
#                row.append(int(temp[i]))
#            field.append(row)
#    end = input()
#    return field

# お邪魔カウントに応じて、盤面にお邪魔ブロックを落とします
def fallObstacle(field, obstacleCount):
    after = copy.deepcopy(field)
    if obstacleCount < width:
        return after
    for j in range(width):
        for i in reversed(range(simulationHeight)):
            if field[i][j] == EMPTY_BLOCK:
                field[i][j] = OBSTACLE_BLOCK
                break
    return after


# 標準エラー出力に盤面の情報を出力します
def printField(field):
    print('\n'.join(map(lambda row: ' '.join(map(lambda block: '{:>2}'.format(block), row)), field)), file=sys.stderr)
    sys.stdout.flush()


def main():
    # AIの名前を出力する
    print(AI_NAME)
    sys.stdout.flush()
    random.seed(123456)

    # ゲーム情報の取得
    for _ in range(maxTurn):
        packs.append(inputPack())

    # ターンの処理
    try:
        while True:
            # 1ターン分のデータを受け取る
            turn = int(input())

            millitime = int(input())
            obstacleCount = int(input())
            skill = int(input())
            score = int(input())
            field = inputField()
            field = fallObstacle(field, obstacleCount)

            enemyMillitime = int(input())
            enemyObstacleCount = int(input())
            enemySkill = int(input())
            enemyScore = int(input())
            enemyField = inputField()
            enemyField = fallObstacle(enemyField, enemyObstacleCount)

            # 操作を決定する
            rotation = random.randrange(0, 4)
            pack = packs[turn]
            pack = rotate(pack, rotation)
            left = 0
            right = width - packSize + 1
            position = random.randrange(left, right)

            print("turn: " + str(turn), file=sys.stderr)
            printPack(pack)
            printField(field)

            # 出力する
            print(position, rotation)
            sys.stdout.flush()

    except Exception as e:
        print("error: {0}".format(e), file=sys.stderr)


if __name__ == '__main__':
    main()
