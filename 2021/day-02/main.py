import argparse

class Submarine:
    depth = 0
    position = 0
    aim = 0

    __include_aim = False

    def __init__(self, include_aim: bool):
        self.__include_aim = include_aim

    def run_command(self, command: str):
        direction, units = command.split(" ", 1)
        units = int(units)

        if self.__include_aim:
            self.__run_with_aim(direction, units)
        else:
            self.__run(direction, units)


    """ Part 1 logic """
    def __run(self, direction: str, units: int):
        if direction == "forward":
            self.position += units

        if direction == "up":
            self.depth -= units

        if direction == "down":
            self.depth += units


    """ Part 2 logic """
    def __run_with_aim(self, direction: str, units: int):
        if direction == "forward":
            self.position += units
            self.depth += self.aim * units

        if direction == "up":
            self.aim -= units

        if direction == "down":
            self.aim += units


def get_commands(filename: str):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file', help='Input file', type=str)
    parser.add_argument('--part-2', help='Use sliding window for measurements', action=argparse.BooleanOptionalAction)
    args = parser.parse_args()

    commands = get_commands(args.input_file)

    sub = Submarine(args.part_2)
    for command in commands:
        sub.run_command(command)

    print(sub.depth * sub.position)


if __name__ == "__main__":
    main()
