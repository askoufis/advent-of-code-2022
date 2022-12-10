struct CPU {
    x: usize,
    cycle: usize,
    signal_cycles: Vec<usize>,
    strength_sum: usize,
    crt_width: usize,
}

#[derive(Debug)]
enum Instruction {
    Addx { v: isize },
    Noop,
}

impl CPU {
    fn new() -> Self {
        let signal_cycles = vec![20, 60, 100, 140, 180, 220];
        Self {
            x: 1,
            cycle: 1,
            signal_cycles,
            strength_sum: 0,
            crt_width: 40,
        }
    }

    fn cycle_to_x(&self) -> usize {
        let cycle_x_pos = self.cycle % self.crt_width - 1;
        cycle_x_pos
    }

    fn draw(&self) {
        let cycle_x_pos = self.cycle_to_x();
        if cycle_x_pos == 0 {
            print!("\n");
        }
        let x = self.x;

        let visible_xs = vec![x - 1, x, x + 1];
        if visible_xs.contains(&cycle_x_pos) {
            print!("#");
        } else {
            print!(".");
        }
    }

    fn tick(&mut self) {
        if self.signal_cycles.contains(&self.cycle) {
            let signal_strength = self.signal_strength();
            self.strength_sum += signal_strength;
        };
        self.draw();
        self.cycle += 1;
    }

    fn tick_n(&mut self, n: usize) {
        for _ in 0..n {
            self.tick()
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Addx { v } => {
                self.tick_n(2);
                self.x = (self.x as isize + v) as usize;
            }
            Instruction::Noop => self.tick_n(1),
        }
    }

    fn signal_strength(&self) -> usize {
        self.cycle * (self.x as usize)
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            if (parts.len() > 1) {
                let v = parts[1].parse().expect("Failed to parse");
                Instruction::Addx { v }
            } else {
                Instruction::Noop
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut cpu = CPU::new();

    input
        .iter()
        .for_each(|instruction| cpu.execute(instruction));

    cpu.strength_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_test() {
        let input = input_generator(INPUT_STR);
        assert_eq!(part1(&input), 13140);
    }
}
