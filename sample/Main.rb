# -*- coding: utf-8 -*-

@AI_NAME = "SampleAI.rb"
@width = 10
@height = 16
@packSize = 2
@summation = 10
@maxTurn = 500
@simulationHeight = (@height + @packSize) + 1
@obstacleBlock = @summation + 1
@emptyBlock = 0
rand = Random.new(12345678)
packs = []

# 標準入力からパックを得ます
def inputPack
    blocks = []
    @packSize.times do
        blocks.push($stdin.gets.strip.split(" ").map {|blcok| blcok.to_i})
    end
    $stdin.gets # END
    return blocks
end

# パックを90度回転させます
def rotateOnce(pack)
    rotated = pack.map {|row| row.dup}
    @packSize.times do |i|
        @packSize.times do |j|
            rotated[j][@packSize - 1 - i] = pack[i][j]
        end
    end
    return rotated
end

# パックを指定した回数だけ90度回転させます
def rotate(pack, rotation)
    rotated = pack
    rotation.times do
        rotated = rotateOnce(rotated)
    end
    return rotated
end

# 標準エラー出力にパックの情報を出力します
def printPack(pack)
    $stderr.puts(
        pack
            .map {|row|
                row.map {|block|
                    sprintf("%2d", block)
                }.join(" ")
            }.join("\n")
    )
    $stderr.flush
end

# 標準入力から盤面を得ます
def inputBoard
    blocks = []
    (@simulationHeight - @height).times do
        row = []
        @width.times do
            row.push(@emptyBlock)
        end
        blocks.push(row)
    end
    @height.times do
        blocks.push($stdin.gets.strip.split(" ").map {|blcok| blcok.to_i})
    end
    $stdin.gets # END
    return blocks
end

# お邪魔カウントに応じて、盤面にお邪魔ブロックを落とします
def fallObstacle(board, obstacleCount)
    after = board.map {|row| row.dup}
    if obstacleCount < @width
        return after
    end
    @width.times do |j|
        (@simulationHeight - 1).downto(0) do |i|
            if after[i][j] == @emptyBlock
                after[i][j] = @obstacleBlock
                break
            end
        end
    end
    return after
end

# 標準エラー出力に盤面の情報を出力します
def printBoard(board)
    $stderr.puts(
        board
            .map {|row|
                row.map {|block|
                    sprintf("%2d", block)
                }.join(" ")
            }.join("\n")
    )
    $stderr.flush
end

# AIの名前を出力する
puts @AI_NAME
$stdout.flush

# ゲーム情報の取得
@maxTurn.times do
    packs.push(inputPack())
end

# ターンの処理
@maxTurn.times do |x|
    # 1ターン分のデータを受け取る
    turn = $stdin.gets.to_i

    millitime = $stdin.gets.to_i
    obstacleCount = $stdin.gets.to_i
    skill = $stdin.gets.to_i
    score = $stdin.gets.to_i
    board = inputBoard()
    board = fallObstacle(board, obstacleCount)

    enemyMillitime = $stdin.gets.to_i
    enemyObstacleCount = $stdin.gets.to_i
    enemySkill = $stdin.gets.to_i
    enemyScore = $stdin.gets.to_i
    enemyBoard = inputBoard()
    enemyBoard = fallObstacle(enemyBoard, enemyObstacleCount)

    # 操作を決定する
    rotation = rand.rand(4)
    pack = packs[turn]
    pack = rotate(pack, rotation)
    left = 0
    right = @width - @packSize
    position = rand.rand(left..right)

    $stderr.puts "turn : #{turn}"
    printPack(pack)
    printBoard(board)

    # 出力する
    puts "#{position} #{rotation}"
    $stdout.flush
end
