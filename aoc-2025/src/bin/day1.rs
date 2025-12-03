use aoc_utils::read_input;

#[derive(Debug)]
struct SafeInstruction{
    direction: char,
    amount: i16,
}

impl SafeInstruction {
    fn new(raw: &str) -> SafeInstruction {
        SafeInstruction {
            direction: raw.chars().nth(0).unwrap(),
            amount: raw[1..=raw.len()-1].parse::<i16>().unwrap()
        }
    }

    fn calculate(&self, initial: i16) -> i16 {
        match self.direction {
            'R' => (initial + self.amount).rem_euclid(100),
            'L' => (initial - self.amount).rem_euclid(100),
            _ => panic!()
        }
    }
}

fn calculate_part_1(instructions: Vec<SafeInstruction>) -> i16 {
    let mut safe_val = 50;
    let mut counter:i16 = 0;
    for instruction in instructions {
        safe_val = instruction.calculate(safe_val);
        if safe_val == 0 {
            counter += 1;
        }
    }
    counter
}


fn calculate_part_2(instructions: Vec<SafeInstruction>) -> i32 {
    let mut safe_val = 50;
    let mut counter:i32 = 0;
    let mut additional:i32 = 0;
    for instruction in instructions {
        println!("{:?}", instruction);
        additional += additional_zeroes(safe_val, instruction.amount, instruction.direction) as i32;
        safe_val = instruction.calculate(safe_val);
        if safe_val == 0 {
            counter += 1;
        }
    }
    counter + additional
}


fn additional_zeroes(start: i16, distance: i16, direction: char) -> i16 {
    if distance == 0 { return 0; }

    if start == 0 && distance < 100 { return 0; }

    let distance_to_zero = if start == 0 {
        100
    } else {
        match direction {
            'R' => 100 - start,
            'L' => start,
            _ => panic!("Invalid direction"),
        }
    };

    if distance <= distance_to_zero {
        return 0;
    }

    // We pass zero at least once
    let remaining = distance - distance_to_zero;

    // don't count the final position if we land on 0
    if remaining % 100 == 0 {
        remaining / 100
    } else {
        1 + remaining / 100
    }
}


#[cfg(test)]
mod test {
    use aoc_utils::read_input;
    use super::*;
    #[test]
    fn test_part_1() {
        // Build the path relative to the crate directory (aoc-2025)
        // The input directory is at the workspace root: <workspace>/input/test_1_1.txt
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_1_1.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let instructions = input.iter().map(|s| SafeInstruction::new(&s)).collect::<Vec<SafeInstruction>>();

        assert_eq!(3, calculate_part_1(instructions));
    }

    #[test]
    fn test_part_2() {
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_1_1.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let instructions = input.iter().map(|s| SafeInstruction::new(&s)).collect::<Vec<SafeInstruction>>();

        assert_eq!(6, calculate_part_2(instructions));
    }

    #[test]
    fn test_additional_zeroes_ez() {
        assert_eq!(1, additional_zeroes(10, 110, 'R'));
    }

    #[test]
    fn test_additional_zeroes_short() {
        assert_eq!(1, additional_zeroes(95, 10, 'R'));
    }

    #[test]
    fn test_random_additional_zeroes() {
        assert_eq!(1, additional_zeroes(0,200, 'L' ))
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/1_1.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let instructions = input.iter().map(|s| SafeInstruction::new(&s)).collect::<Vec<SafeInstruction>>();
    println!("Part 1: {}", calculate_part_1(instructions));
}

fn part_2() {

    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/1_1.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let instructions = input.iter().map(|s| SafeInstruction::new(&s)).collect::<Vec<SafeInstruction>>();
    println!("Part 2: {}", calculate_part_2(instructions));
}