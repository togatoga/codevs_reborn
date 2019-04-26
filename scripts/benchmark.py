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
def cmd(solver, pack, info, num):
    files = [os.path.join(pack, p) for p in os.listdir(pack)]
    files.sort()
    files = files[:num]
    assert(len(files) == num)
    print("Start")
    with concurrent.futures.ThreadPoolExecutor(max_workers=multiprocessing.cpu_count()) as executor:
        solver_name = os.path.basename(solver)
        now = datetime.datetime.now()
        now = now.strftime("%Y%m%d_%H%M%S")
        output_dir = f'data/time/{now}'
        os.makedirs(output_dir)
        for file in files:
            file_name = os.path.basename(file)
            exec_cmd = f'{solver} bench --pack {file} --info {info}'
            executor.submit(task, exec_cmd)
    print("Done!!")


def main():
    cmd()


if __name__ == "__main__":
    main()
