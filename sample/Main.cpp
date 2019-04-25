/**
 * コンパイル： g++ -std=c++11 Main.cpp
 */
#include <algorithm>
#include <iostream>
#include <random>
#include <vector>
using namespace std;

static const string AI_NAME = "SampleAI.cpp";
static const int width = 10;
static const int height = 16;
static const int summation = 10;
static const int packSize = 2;
static const int maxTurn = 500;
static const int simulationHeight = height + packSize + 1;
static const int OBSTACLE_BLOCK = summation + 1;
static const int EMPTY_BLOCK = 0;

mt19937 MT(8410325);
int randInt(int from, int to) {
  uniform_int_distribution<int> rand(from, to - 1);
  return rand(MT);
}

// 標準入力からパックを得ます
vector<vector<int>> inputPack() {
  vector<vector<int>> pack;
  for (int i = 0; i < packSize; i++) {
    vector<int> row;
    for (int j = 0; j < packSize; j++) {
      int block;
      cin >> block;
      row.push_back(block);
    }
    pack.push_back(row);
  }
  string endStr;
  cin >> endStr;
  return pack;
}

// パックを90度回転させます
vector<vector<int>> rotateOnce(vector<vector<int>> pack) {
  vector<vector<int>> rotated = pack;
  vector<vector<int>>(packSize, vector<int>(packSize, 0));
  for (int i = 0; i < packSize; i++) {
    for (int j = 0; j < packSize; j++) {
      rotated[j][packSize - 1 - i] = pack[i][j];
    }
  }
  return rotated;
}

// パックを指定した回数だけ90度回転させます
vector<vector<int>> rotate(vector<vector<int>> pack, int rotation) {
  for (int r = 0; r < rotation; r++) {
    pack = rotateOnce(pack);
  }
  return pack;
}

// 標準エラー出力にパックの情報を出力します
void printPack(vector<vector<int>> pack) {
  for (int i = 0; i < packSize; i++) {
    for (int j = 0; j < packSize; j++) {
      char s[4];
      snprintf(s, 4, "%1s%2d", (j == 0 ? "" : " "), pack[i][j]);
      cerr << s;
    }
    cerr << endl;
  }
  cerr.flush();
}

// 標準入力から盤面を得ます
vector<vector<int>> inputField() {
  vector<vector<int>> field;
  for (int i = 0; i < simulationHeight - height; i++) {
    vector<int> row;
    for (int j = 0; j < width; j++) {
      row.push_back(EMPTY_BLOCK);
    }
    field.push_back(row);
  }
  for (int i = 0; i < height; i++) {
    vector<int> row;
    for (int j = 0; j < width; j++) {
      int block;
      cin >> block;
      row.push_back(block);
    }
    field.push_back(row);
  }
  string endStr;
  cin >> endStr;
  return field;
}

// お邪魔カウントに応じて、盤面にお邪魔ブロックを落とします
vector<vector<int>> fallObstacle(vector<vector<int>> field, int obstacleCount) {
  vector<vector<int>> after = field;
  if (obstacleCount < width)
    return after;
  for (int j = 0; j < width; j++) {
    for (int i = simulationHeight - 1; i >= 0; i--) {
      if (after[i][j] == EMPTY_BLOCK) {
        after[i][j] = OBSTACLE_BLOCK;
        break;
      }
    }
  }
  return after;
}

// 標準エラー出力に盤面の情報を出力します
void printField(vector<vector<int>> field) {
  for (int i = 0; i < simulationHeight; i++) {
    for (int j = 0; j < width; j++) {
      char s[4];
      snprintf(s, 4, "%s%2d", (j == 0 ? "" : " "), field[i][j]);
      cerr << s;
    }
    cerr << endl;
  }
  cerr.flush();
}

int main() {
  // AIの名前を出力する
  cout << AI_NAME << endl;
  cout.flush();

  // ゲーム情報の取得
  vector<vector<vector<int>>> packs;
  for (int i = 0; i < maxTurn; i++) {
    packs.push_back(inputPack());
  }

  int turn;

  int millitime;
  int obstacleCount;
  int skill;
  int score;

  int enemyObstacleCount;
  int enemyMillitime;
  int enemySkill;
  int enemyScore;

  // ターンの処理
  for (int i = 0; i < maxTurn; i++) {

    // 1ターン分のデータを受け取る
    cin >> turn;

    cin >> millitime;
    cin >> obstacleCount;
    cin >> skill;
    cin >> score;
    vector<vector<int>> field = inputField();
    field = fallObstacle(field, obstacleCount);

    cin >> enemyMillitime;
    cin >> enemyObstacleCount;
    cin >> enemySkill;
    cin >> enemyScore;
    vector<vector<int>> enemyField = inputField();
    enemyField = fallObstacle(enemyField, enemyObstacleCount);

    // 操作を決定する
    int rotation = randInt(0, 4);
    vector<vector<int>> pack = packs[turn];
    pack = rotate(pack, rotation);
    int position = randInt(0, width - packSize + 1);

    cerr << "turn : " << turn << endl;
    cerr.flush();
    printPack(pack);
    printField(field);

    // 出力する
    cout << position << " " << rotation << endl;
    cout.flush();
  }
}
