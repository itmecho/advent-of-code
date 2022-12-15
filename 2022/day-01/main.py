from itertools import groupby
import sys

with open(sys.argv[1], 'r') as f:
    lines = [line.strip() for line in f.readlines()]
    elves = [sum(map(int, elf))
             for value, elf in groupby(lines, key=bool) if value]

    answer = max(elves)
    print(f"part 1: {answer}")

    elves.sort(reverse=True)
    answer = sum(elves[0:3])
    print(f"part 2: {answer}")
