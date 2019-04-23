if __name__ == "__main__":
    board = []
    for y in range(16):
        res = []
        v = input().split()
        line = ",".join(v)
        board.append(f"[{line}]")
    print(",\n".join(board))
