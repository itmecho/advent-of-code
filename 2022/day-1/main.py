from itertools import groupby
import time
import sys

def timing(f):
    def wrap(*args, **kwargs):
        time1 = time.time()
        ret = f(*args, **kwargs)
        time2 = time.time()
        print('{:s} function took {:.3f} ms'.format(f.__name__, (time2-time1)*1000.0))

        return ret
    return wrap

@timing
def gross(f):
    elves = [sum(map(int, list(elf))) for val, elf in groupby([line.strip() for line in f.readlines()], key = bool) if val]
    elves.sort(reverse=True)
    print(f"part 1: {elves[0]}")
    print(f"part 2: {sum(elves[0:3])}")

@timing
def better(f):
    elves = [0]
    lines = f.readlines()
    elf = 0
    for line in lines:
        line = line.strip()
        if line == '':
            elf += 1
            elves.append(0)
        else:
            elves[elf] += int(line)

    elves.sort(reverse=True)

    print(f"part 1: {elves[0]}")
    print(f"part 2: {sum(elves[0:3])}")

with open(sys.argv[1]) as f:
    better(f)

with open(sys.argv[1]) as f:
    gross(f)
