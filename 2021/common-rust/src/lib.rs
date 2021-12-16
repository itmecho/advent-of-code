use std::{env::args, fs::File, io::Read};

pub struct Cli {
    input_file: File,
    mode: Mode,
    part_1: fn(&mut dyn Read),
    part_2: fn(&mut dyn Read),
}

impl Cli {
    pub fn new(part_1: fn(&mut dyn Read), part_2: fn(&mut dyn Read)) -> Self {
        let mut mode = Mode::Part1;
        let mut input_file_name = "input.txt";
        for arg in args().skip(1) {
            match arg.as_str() {
                "--part-2" => mode = Mode::Part2,
                "--test" => input_file_name = "input.test.txt",
                arg => panic!("unknown argument {}", arg),
            }
        }

        let input_file =
            File::open(input_file_name).expect(&format!("loading input file {}", input_file_name));

        Self {
            input_file,
            mode,
            part_1,
            part_2,
        }
    }

    pub fn run(&mut self) {
        let part = match self.mode {
            Mode::Part1 => {
                println!("running part 1");
                self.part_1
            }
            Mode::Part2 => {
                println!("running part 2");
                self.part_2
            }
        };

        part(&mut self.input_file);
    }
}

enum Mode {
    Part1,
    Part2,
}
