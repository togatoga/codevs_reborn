import java.util.Arrays;
import java.util.Random;
import java.util.Scanner;
import java.util.stream.Collectors;

public class Main {

    public static void main(String[] args) {
        new Main().run();
    }

    static final String AI_NAME = "SampleAI.java";

    Random random = new Random();

    static final int width = 10;

    static final int height = 16;

    static final int packSize = 2;

    static final int summation = 10;

    static final int maxTurn = 500;

    static int simulationHeight = height + packSize + 1;

    static final int EMPTY_BLOCK = 0;

    static int OBSTACLE_BLOCK = summation + 1;

    // 標準入力からパックを得ます
    int[][] inputPack(Scanner in) {
        int[][] pack = new int[packSize][packSize];
        for (int i = 0; i < packSize; i++) {
            for (int j = 0; j < packSize; j++) {
                pack[i][j] = in.nextInt();
            }
        }
        in.next(); // END
        return pack;
    }

    // パックを90度回転させます
    int[][] rotateOnce(int[][] pack) {
        int[][] rotated = new int[packSize][packSize];
        for (int i = 0; i < packSize; i++) {
            for (int j = 0; j < packSize; j++) {
                rotated[j][packSize - 1 - i] = pack[i][j];
            }
        }
        return rotated;
    }

    // パックを指定した回数だけ90度回転させます
    int[][] rotate(int[][] pack, int rotation) {
        for (int r = 0; r < rotation; r++) {
            pack = rotateOnce(pack);
        }
        return pack;
    }

    // 標準エラー出力にパックの情報を出力します
    void printPack(int[][] pack) {
        System.err.println(
            Arrays.stream(pack).map(row -> {
                return Arrays.stream(row)
                    .mapToObj(block -> String.format("%2d", block))
                    .collect(Collectors.joining(" "));
            }).collect(Collectors.joining("\n"))
        );
        System.err.flush();
    }

    // 標準入力から盤面を得ます
    int[][] inputField(Scanner in) {
        int[][] field = new int[simulationHeight][width];

        for (int i = 0; i < simulationHeight - height; i++) {
            for (int j = 0; j < width; j++) {
                field[i][j] = EMPTY_BLOCK;
            }
        }
        for (int i = simulationHeight - height; i < simulationHeight; i++) {
            for (int j = 0; j < width; j++) {
                field[i][j] = in.nextInt();
            }
        }
        in.next(); // END
        return field;
    }

    // お邪魔カウントに応じて、盤面にお邪魔ブロックを落とします
    int[][] fallObstacle(int[][] field, int obstacleCount) {
        int[][] after = Arrays.stream(field)
            .map(row -> Arrays.copyOf(row, width))
            .toArray(int[][]::new);
        if (obstacleCount < width) return after;
        for (int j = 0; j < width; j++) {
            for (int i = simulationHeight - 1; i >= 0; i--) {
                if (field[i][j] == EMPTY_BLOCK) {
                    field[i][j] = OBSTACLE_BLOCK;
                    break;
                }
            }
        }
        return after;
    }

    // 標準エラー出力に盤面の情報を出力します
    void printField(int[][] field) {
        System.err.println(
            Arrays.stream(field).map(row -> {
                return Arrays.stream(row)
                    .mapToObj(block -> String.format("%2d", block))
                    .collect(Collectors.joining(" "));
            }).collect(Collectors.joining("\n"))
        );
        System.err.flush();
    }


    void run() {
        // AIの名前を出力する
        System.out.println(AI_NAME);
        System.out.flush();

        try (Scanner in = new Scanner(System.in)) {

            // ゲーム情報の取得
            int[][][] packs = new int[maxTurn][][];
            for (int i = 0; i < maxTurn; i++) {
                packs[i] = inputPack(in);
            }

            // ターンの処理
            while (true) {
                // 1ターン分のデータを受け取る
                int turn = in.nextInt();

                int millitime = in.nextInt();
                int obstacleCount = in.nextInt();
                int skill = in.nextInt();
                int score = in.nextInt();
                int[][] field = inputField(in);
                field = fallObstacle(field, obstacleCount);

                int enemyMillitime = in.nextInt();
                int enemyObstacleCount = in.nextInt();
                int enemySkill = in.nextInt();
                int enemyScore = in.nextInt();
                int[][] enemyField = inputField(in);
                enemyField = fallObstacle(enemyField, enemyObstacleCount);

                // 操作を決定する
                int rotation = random.nextInt(4);
                int[][] pack = packs[turn];
                pack = rotate(pack, rotation);
                int left = 0;
                int right = width - packSize + 1;
                int position = random.nextInt(right - left) + left;

                System.err.println("turn : " + turn);
                System.err.flush();
                printPack(pack);
                printField(field);

                // 操作を出力する
                System.out.println(position + " " + rotation);
                System.out.flush();
            }
        }
    }

}
