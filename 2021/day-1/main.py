import argparse
from typing import List


def get_report(filename: str) -> List[int]:
    with open(filename) as f:
        return [int(line.strip()) for line in f.readlines()]


def count_increases(report):
    count = 0

    for idx in range(1, len(report)):
        if report[idx] > report[idx - 1]:
            count = count + 1

    return count


def count_increases_with_window(report):
    count = 0

    for idx in range(3, len(report)):
        current = sum(report[idx-2:idx+1])
        previous = sum(report[idx-3:idx])
        if current > previous:
            count = count + 1

    return count


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file', help='Input file', type=str)
    parser.add_argument('--part-2', help='Use sliding window for measurements', action=argparse.BooleanOptionalAction)

    args = parser.parse_args()

    report = get_report(args.input_file)

    if args.part_2:
        result = count_increases_with_window(report)
    else:
        result = count_increases(report)

    print(result)


if __name__ == "__main__":
    main()
