import subprocess
import click
import os
import concurrent.futures
import multiprocessing
import datetime


def task(exec_cmd):
    subprocess.run(exec_cmd, shell=True)


@click.command()
@click.argument("solver", required=True)
@click.argument("pack", required=True)
@click.argument("info", required=True)
@click.option("--num", "-n", default=500)
@click.option("--seed", "-s", default=1024)
def cmd(solver, pack, info, num, seed):
    files = [os.path.join(pack, p) for p in os.listdir(pack)]
    files.sort()
    files = files[:num]
    assert(len(files) == num)
    print("Start")
    with concurrent.futures.ThreadPoolExecutor(max_workers=multiprocessing.cpu_count()) as executor:
        now = datetime.datetime.now()
        now = now.strftime("%Y%m%d_%H%M%S")
        output_dir = f'data/result/{now}'
        print(f'Create a directory: {output_dir}')
        os.makedirs(output_dir)

        for file in files:
            file_name = f'{os.path.basename(solver)}_{os.path.basename(file)}_{os.path.basename(info)}_result'
            exec_cmd = f'{solver} bench --pack {file} --info {info} --output {output_dir}/{file_name}.csv --seed {seed}'
            try:
                executor.submit(task, exec_cmd)
            except KeyboardInterrupt:
                executor._threads.clear()
                concurrent.futures.thread._threads_queues.clear()
                raise
    print("Done!!")


def main():
    cmd()


if __name__ == "__main__":
    main()
