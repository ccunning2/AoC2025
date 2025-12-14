use std::collections::HashSet;
use aoc_utils::read_input;

fn main() {

    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/5.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let mut vals = parse_part_1(input);
    println!("Part 1: {}", calculate_part_1(&vals.0, &mut vals.1));
    //Condense the ranges for part 2
    condense_ranges(&mut vals.0);
    println!("Part 2: {}", calculate_part_2(&vals.0));
}

fn calculate_part_1(ranges: &Vec<IngredientRange>, ingredients: &mut HashSet<i64>) -> i64 {
    let mut count = 0;
    for range in ranges.iter() {
        count += range.get_matches_part_1(ingredients);
    }
    count
}

fn calculate_part_2(ranges: &Vec<IngredientRange>) -> i64 {
    let mut count = 0;
    for range in ranges.iter() {
        count += (range.range_high - range.range_low) + 1;
    }
    count
}
fn condense_ranges(ranges: &mut Vec<IngredientRange>) {
    ranges.sort_by_key(|a| (a.range_low, a.range_high));

    let mut result: Vec<IngredientRange> = Vec::new();
    result.push(ranges[0].clone());

    for range in ranges.iter().skip(1) {
        let last = result.last_mut().unwrap();
        if range.range_low <= last.range_high {
            last.range_high = last.range_high.max(range.range_high);
        } else {
            result.push(range.clone());
        }
    }

    *ranges = result;
}
#[derive(Debug,Clone)]
struct IngredientRange {
    range_low: i64,
    range_high: i64
}

impl IngredientRange {
    fn new(range_low: i64, range_high: i64) -> Self {
        Self {
            range_low,
            range_high
        }
    }

    fn get_matches_part_1(&self, n: &mut HashSet<i64>) -> i64 {
        let remove_me: Vec<_> = n.iter()
            .filter(|&n| n >= &self.range_low && n <= &self.range_high)
            .copied()
            .collect();

        for x in &remove_me {
            n.remove(&x);
        }
        remove_me.len() as i64
    }
}

fn parse_part_1(input: Vec<String>) -> (Vec<IngredientRange>, HashSet<i64>) {
    let mut ingredients: bool = false;
    let mut ingredient_list: HashSet<i64> = HashSet::new();
    let mut ingrediant_ranges: Vec<IngredientRange> = Vec::new();
    for s in input.iter() {
        if s.is_empty() {
            ingredients = true;
            continue;
        }
        if ingredients {
            ingredient_list.insert(s.parse::<i64>().unwrap());
        } else {
            let ranges = s.split("-").collect::<Vec<&str>>();
            let low = ranges.get(0).unwrap().parse::<i64>().unwrap();
            let high = ranges.get(1).unwrap().parse::<i64>().unwrap();
            ingrediant_ranges.push(IngredientRange::new(low, high));
        }

    }
    (ingrediant_ranges, ingredient_list)
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
            .join("../input/test_5.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let mut vals = parse_part_1(input);
        assert_eq!(3, calculate_part_1(&vals.0, &mut vals.1));
    }

    #[test]
    fn test_part_2() {
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/test_5.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let mut vals = parse_part_1(input);
        //Need to condense overlapping ranges
         condense_ranges(&mut vals.0);
        assert_eq!(14, calculate_part_2(&vals.0));
    }
}