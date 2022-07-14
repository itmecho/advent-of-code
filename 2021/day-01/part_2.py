import sys
from typing import List


def get_report(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(line.strip()) for line in f.readlines()]

def main():
    report = get_report(sys.argv[1])
    increases = 0

    for idx in range(3, len(report)):
        current = sum(report[idx-2:idx+1])
        previous = sum(report[idx-3:idx])
        if current > previous:
            increases = increases + 1

    print(increases)


if __name__ == "__main__":
    main()
