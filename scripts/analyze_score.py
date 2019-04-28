import sys
import argparse
import os
import csv
import statistics
import seaborn as sns
import matplotlib.pyplot as plt
from collections import OrderedDict


def read_score_file(file_path):
    with open(file_path, "r") as f:
        reader = list(csv.DictReader(f))
        assert(len(reader) == 1)
        content = reader[0]
        return content


def analyze_score(cumulative_game_scores, file_path):
    mean = statistics.mean(cumulative_game_scores)
    median = statistics.median(cumulative_game_scores)
    min_score = min(cumulative_game_scores)
    max_score = max(cumulative_game_scores)
    with open(file_path, "w") as f:
        f.write(f'Number   : {len(cumulative_game_scores)}\n')
        f.write(f'min_score: {min_score}\n')
        f.write(f'max_score: {max_score}\n')
        f.write(f'mean     : {mean}\n')
        f.write(f'median   : {median}\n')
    print(f'Number   : {len(cumulative_game_scores)}')
    print(f'min_score: {min_score}')
    print(f'max_score: {max_score}')
    print(f'mean     : {mean}')
    print(f'median   : {median}')


def plot_hist_score(cumulative_game_scores, file_path):
    plt.subplot(2, 1, 1)
    plt.xticks(range(0,
                     max(cumulative_game_scores), 10))
    sns.distplot(cumulative_game_scores, kde=False, rug=False,
                 bins=50, color='red', axlabel="game score", hist_kws={'edgecolor': 'black'})
    plt.title("A Histgram of cumulative game score")
    plt.subplot(2, 1, 2)
    sns.distplot(cumulative_game_scores, bins=100, color='blue', hist_kws={
                 'cumulative': True, 'edgecolor': 'black'}, kde=False, rug=False)
    plt.title("A Cumulative histgram of cumulative game score")
    plt.savefig(file_path)

    plt.show()


def main(args):
    root = args.score
    limit = args.number
    contents = OrderedDict()
    # read score files order by the file name
    for idx, file in enumerate(sorted(os.listdir(root))):
        if idx >= limit:
            break
        file_name = os.path.basename(file)
        file_path = os.path.join(root, file)
        contents[file_name] = read_score_file(file_path)
    # analyze
    cumulative_game_scores = []
    for file_name, content in contents.items():
        cumulative_game_scores.append(int(content['cumulative_game_score']))

    output_path = f'data/analysis/'
    os.makedirs(output_path, exist_ok=True)
    file_name = os.path.basename(root)
    analyze_score(cumulative_game_scores, os.path.join(
        output_path, f'{file_name}.txt'))
    plot_hist_score(cumulative_game_scores, os.path.join(
        output_path, f'{file_name}.png'))
    return os.EX_OK


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Analyze score files")
    parser.add_argument("score", help="score directory path")
    parser.add_argument(
        '-n', '--number', type=int, help="the number of score files which are analyzed", default=500)
    args = parser.parse_args()
    sys.exit(main(args))
