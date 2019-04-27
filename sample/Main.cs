using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace codevsSampleAI {
class Program {
  const string AI_NAME = "SampleAI.cs";
  const int EMPTY_BLOCK = 0;
  Random random = new Random(12345678);
  int turn = -1;

  int[][, ] pack;

  const int width = 10;

  const int height = 16;

  const int packSize = 2;

  static int simulationHeight = height + packSize + 1;

  const int summation = 10;

  static int obstacle = summation + 1;

  const int maxTurn = 500;

  static void Main(string[] args) { new Program().run(); }

private
  string ReadLine() {
    for (;;) {
      string line = Console.ReadLine();
      if (line == null)
        throw new Exception("EOF");
      if (line.Trim().Length == 0)
        continue;
      return line;
    }
  }

  void println(string msg) {
    Console.WriteLine(msg);
    Console.Out.Flush();
  }

  void debug(string msg) {
    Console.Error.WriteLine(msg);
    Console.Error.Flush();
  }

  // 標準入力からパックを得ます
  int[, ] inputPack() {
    int[, ] pack = new int[packSize, packSize];
    for (int i = 0; i < packSize; ++i) {
      String[] line = ReadLine().Split();
      for (int j = 0; j < packSize; ++j)
        pack[i, j] = int.Parse(line[j]);
    }
    ReadLine(); // END
    return pack;
  }

  // パックを90度回転させます
  int[, ] rotateOnce(int[, ] pack) {
    int[, ] rotated = (int[, ])pack.Clone();
    for (int i = 0; i < packSize; ++i)
      for (int j = 0; j < packSize; ++j)
        rotated[j, packSize - i - 1] = pack[i, j];
    return rotated;
  }

  // パックを指定した回数だけ90度回転させます
  int[, ] rotate(int[, ] pack, int rotation) {
    int[, ] rotated = (int[, ])pack.Clone();
    for (int i = 0; i < rotation; ++i)
      rotated = rotateOnce(rotated);
    return rotated;
  }

  // 標準エラー出力にパックの情報を出力します
  void printPack(int[, ] pack) {
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < packSize; ++i) {
      sb.Append(i == 0 ? "" : Environment.NewLine);
      for (int j = 0; j < packSize; ++j) {
        sb.Append(j == 0 ? "" : " ")
            .Append(pack[i, j].ToString().PadLeft(2, ' '));
      }
    }
    debug(sb.ToString());
  }

  // 標準入力から盤面を得ます
  int[, ] inputBoard() {
    int[, ] board = new int[simulationHeight, width];
    for (int i = 0; i < simulationHeight - height - 1; ++i) {
      for (int j = 0; j < width; ++j)
        board[i, j] = EMPTY_BLOCK;
    }
    for (int i = simulationHeight - height; i < simulationHeight; ++i) {
      string[] line = ReadLine().Split();
      for (int j = 0; j < width; ++j)
        board[i, j] = int.Parse(line[j]);
    }
    ReadLine(); // END
    return board;
  }

  // お邪魔カウントに応じて、盤面にお邪魔ブロックを落とします
  int[, ] fallObstacle(int[, ] board, int obstacleCount) {
    int[, ] after = (int[, ])board.Clone();
    if (obstacleCount < width)
      return after;
    for (int j = 0; j < width; ++j)
      for (int i = simulationHeight - 1; i >= 0; --i)
        if (after[i, j] == EMPTY_BLOCK) {
          after[i, j] = obstacle;
          break;
        }
    return after;
  }

  // 標準エラー出力に盤面の情報を出力します
  void printBoard(int[, ] board) {
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < simulationHeight; ++i) {
      sb.Append(i == 0 ? "" : Environment.NewLine);
      for (int j = 0; j < width; ++j) {
        sb.Append(j == 0 ? "" : " ")
            .Append(board[i, j].ToString().PadLeft(2, ' '));
      }
    }
    debug(sb.ToString());
  }

  void run() {
    // AIの名前を出力する
    println(AI_NAME);

    // ゲーム情報の取得
    pack = new int[maxTurn][, ];
    for (int i = 0; i < maxTurn; ++i) {
      pack[i] = new int[packSize, packSize];
      for (int j = 0; j < packSize; ++j) {
        String[] line = ReadLine().Split();
        for (int k = 0; k < packSize; ++k)
          pack[i][j, k] = int.Parse(line[k]);
      }
      ReadLine(); // END
    }

    // ターンの処理
    while (true) {
      // 1ターン分のデータを受け取る
      turn = int.Parse(ReadLine());

      long millitime = long.Parse(ReadLine());
      int obstacleCount = int.Parse(ReadLine());
      int skill = int.Parse(ReadLine());
      int score = int.Parse(ReadLine());

      int[, ] board = inputBoard();
      board = fallObstacle(board, obstacleCount);

      long enemyMillitime = long.Parse(ReadLine());
      int enemyObstacleCount = int.Parse(ReadLine());
      int enemySkill = int.Parse(ReadLine());
      int enemyScore = int.Parse(ReadLine());

      int[, ] enemyBoard = inputBoard();
      enemyBoard = fallObstacle(enemyBoard, enemyObstacleCount);

      // 操作を決定する
      int rotation = random.Next(4);
      int[, ] pack = this.pack[turn];
      pack = rotate(pack, rotation);
      int left = 0, right = width - packSize;
      int position = random.Next(left, right + 1);

      debug("turn : " + turn);
      printPack(pack);
      printBoard(board);

      // 出力する
      println(position + " " + rotation);
    }
  }
}
} // namespace codevsSampleAI
