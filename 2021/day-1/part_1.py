from typing import List


def get_report() -> List[int]:
    with open('input.txt') as f:
        return [int(line.strip()) for line in f.readlines()]

def main():
    report = get_report()
    increases = 0

    for idx in range(1, len(report)):
        if report[idx] > report[idx - 1]:
            increases = increases + 1

    print(increases)


if __name__ == "__main__":
    main()
