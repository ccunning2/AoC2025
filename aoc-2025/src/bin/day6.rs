use std::collections::HashMap;
use aoc_utils::read_input;

fn main() {
    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/6.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let problems = parse_problems(&input);
    println!("Part 1:{}", calculate(problems));
    let part_2_problems = parse_problems_2(&input);
    println!("Part 2:{}", calculate(part_2_problems));
}


fn get_operand_groups(operand_row: String) -> Vec<MathProblem> {
    //Will be in the order from right to left
    let mut results: Vec<MathProblem> = Vec::new();
    for (index, c ) in operand_row.chars().rev().enumerate() {
        match c {
            '+' => {results.push(MathProblem::new2(index, Operator::ADD))},
            '*' => {results.push(MathProblem::new2(index, Operator::MUL))},
            _ => {}
        }
    }
    results
}

fn calculate(problems: Vec<MathProblem>) -> i64 {
    problems.iter().map(MathProblem::calculate).sum()
}
enum Operator {
    ADD,
    MUL
}

impl Operator {
    fn new(c:&str) -> Self {
        match c {
            "*" => Self::MUL,
            "+" => Self::ADD,
            _ => panic!("Unknown operator {}", c)
        }
    }
}

struct MathProblem {
    index: usize,
    operands: Vec<i64>,
    operator: Option<Operator>,
}

impl MathProblem {
    fn new(index: usize, first_val:i64) -> MathProblem {
        Self {
            index,
            operands: vec![first_val],
            operator: None,
        }
    }

    fn new2(index: usize, operator: Operator) -> MathProblem {
        Self {
            index,
            operands: Vec::new(),
            operator: Some(operator),
        }
    }

    fn calculate(&self) -> i64 {
        match self.operator {
            Some(Operator::ADD) => return self.operands.iter().sum(),
            Some(Operator::MUL) => return self.operands.iter().product(),
            _ => panic!("Unknown Operator {}", self.index)
        }
    }
}


fn parse_problems(input: &Vec<String>) -> Vec<MathProblem> {
    let mut math_problems : Vec<MathProblem> = Vec::new();
    for line in input.iter() {
        let split: Vec<&str> = line.split_whitespace().collect();
        split.iter().enumerate().for_each(|(index, value)| {
            let actual_val = value.parse::<i64>();
            if let Some(problem) = math_problems.get_mut(index) {
                if actual_val.is_ok() {
                    problem.operands.push(actual_val.unwrap());
                } else {
                    problem.operator = Some(Operator::new(value));
                }
            } else {
                let problem = MathProblem::new(index, actual_val.unwrap());
                math_problems.push(problem);
            }
        })
    }
    math_problems
}

fn parse_problems_2(input: &Vec<String>) -> Vec<MathProblem> {
    //Get the # of columns for each group.
    let operand_string = input.last().unwrap();
    //The lengths, from right to left, that will define the columns for a group
    let mut problems = get_operand_groups(operand_string.to_string());
    //Sort problems by index
    problems.sort_by_key(|problem| problem.index);
    //Build hashs maps of values for each column index (from right to left)
    let col_map: HashMap<usize, String> = build_col_map(&input);
    //Iterate over length of a row, go up to and include the index included in the problem, add each column to operand
    //vector in the problem. Need to skip a space after each problem (except the last)
    let mut problem_iterator = problems.iter_mut();
    let mut current_problem = problem_iterator.next().unwrap();
    for i in 0..input.get(0).unwrap().len() {
        if i > current_problem.index {
            current_problem = problem_iterator.next().unwrap();
        }
        if let Ok(operand) = col_map.get(&i).unwrap().trim().parse::<i64>() {
            current_problem.operands.push(operand);
        }
    }
    problems
}

fn build_col_map(input: &Vec<String>) -> HashMap<usize, String> {
    let mut col_map: HashMap<usize, String> = HashMap::new();
    for  row in input.iter().take(input.len().saturating_sub(1)) {
        for (col_index, col) in row.chars().rev().enumerate() {
            col_map
                .entry(col_index)
                .and_modify(|existing| existing.push_str(col.to_string().as_str()))
                .or_insert_with(|| col.to_string());
        }
    }
    col_map
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
            .join("../input/test_6.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let problems = parse_problems(&input);
        assert_eq!(4277556, calculate(problems));
        }

    #[test]
    fn test_part_2() {
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/test_6.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let problems = parse_problems_2(&input);
        assert_eq!(3263827, calculate(problems));
    }
 }




