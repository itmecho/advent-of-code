class Submarine:
    depth = 0
    position = 0

    def run_command(self, command: str):
        direction, units = command.split(" ", 1)
        units = int(units)

        if direction == "forward":
            self.position += units

        if direction == "up":
            self.depth -= units

        if direction == "down":
            self.depth += units


def get_commands():
    with open("./input.txt") as f:
        return [line.strip() for line in f.readlines()]

def main():
    commands = get_commands()

    sub = Submarine()
    for command in commands:
        sub.run_command(command)

    print(sub.depth * sub.position)


if __name__ == "__main__":
    main()
