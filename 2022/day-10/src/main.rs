use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        Ok(match parts.as_slice() {
            &["noop"] => Self::Noop,
            &["addx", value] => Self::Addx(value.parse().unwrap()),
            _ => panic!("unsupported line: {s}"),
        })
    }
}

struct Machine<'a> {
    value: isize,
    instructions: &'a [Instruction],
    index: usize,
    paused: bool,
    total_cycles: usize,
    screen: [[&'static str; 40]; 6],
    cursor: (usize, usize),
}

impl<'a> Machine<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            value: 1,
            index: 0,
            paused: false,
            total_cycles: 0,
            screen: [["."; 40]; 6],
            cursor: (0, 0),
        }
    }

    fn cycle(&mut self) -> Option<()> {
        let instruction = self.instructions.get(self.index);
        if instruction.is_none() {
            return None;
        }

        self.total_cycles += 1;
        self.draw();

        match instruction.unwrap() {
            Instruction::Noop => self.index += 1,
            Instruction::Addx(_) if !self.paused => self.paused = true,
            Instruction::Addx(value) if self.paused => {
                self.paused = false;
                self.index += 1;
                self.value += value;
            }
            _ => unreachable!(),
        }
        println!(
            "cycle {}: {} {:?}",
            self.total_cycles, self.value, instruction
        );

        Some(())
    }

    fn draw(&mut self) {
        let (row, col) = self.cursor;
        let sprite = [self.value - 1, self.value, self.value + 1];
        if sprite.contains(&(col as isize)) {
            self.screen[row][col] = "#";
        }
        if self.cursor.1 == 39 {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
        } else {
            self.cursor.1 += 1;
        }
    }

    fn print(&self) {
        for row in self.screen {
            for pixel in row {
                print!("{pixel}");
            }
            println!("");
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let mut machine = Machine::new(&instructions);
    let mut cycles = 0;
    let mut score = 0;

    while let Some(_) = machine.cycle() {
        cycles += 1;
        if cycles == 20 || (cycles >= 20 && (cycles - 20) % 40 == 0) {
            score += cycles * machine.value;
        }
    }

    println!("part 1: {}", score);
    println!("part 2:");
    machine.print();
}
