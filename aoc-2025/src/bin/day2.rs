use aoc_utils::read_input;

fn main() {

    // Build the path relative to the crate directory (aoc-2025)
    // The input directory is at the workspace root: <workspace>/input/test_1_1.txt
    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/2.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let ranges = input.get(0).unwrap().split(',').map(|x| IdRange::new(x)).collect::<Vec<IdRange>>();
    println!("Part 1: {}", calculate_part_1(&ranges));
    print!("Part 2: {}", calculate_part_2(&ranges));
}

#[derive(Debug)]
struct IdRange {
    low: i64,
    high: i64
}

impl IdRange {
    fn new(range: &str) -> Self {
        let a: Vec<&str> = range.split("-").collect();
        let low = a.get(0).unwrap().parse::<i64>().unwrap();
        let high = a.get(1).unwrap().parse::<i64>().unwrap();
        Self { low, high }
    }
}


fn calculate_part_1(ranges: &Vec<IdRange>) -> i64 {
    ranges.iter().flat_map(|r| get_duplicated_ranges(r)).sum::<i64>()
}


fn calculate_part_2(ranges: &Vec<IdRange>) -> i64 {
    ranges.iter().flat_map(|r| get_duplicated_ranges2(r)).sum::<i64>()
}


fn get_duplicated_ranges2(range: &IdRange) -> Vec<i64> {
    let mut dupes: Vec<i64> = Vec::new();
    for i in range.low..=range.high {
        let string_val = i.to_string();
        let halfway = string_val.len() / 2;
        for j in 1..=halfway {
            let slice_1 = &string_val[0..j];
            let test = slice_1.repeat(string_val.len()/slice_1.len());
            if test == string_val {
                dupes.push(i);
                break;
            }
        }
    }
    dupes
}

fn get_duplicated_ranges(range: &IdRange) -> Vec<i64> {
    let mut dupes: Vec<i64> = Vec::new();
    for i in range.low..=range.high {
        let string_val = i.to_string();
        let halfway = string_val.len() / 2;
        let a= &string_val[0..halfway];
        let b = &string_val[halfway..];
        if a == b {
            dupes.push(i);
        }

    }
    dupes
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
            .join("../input/test_2.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let ranges = input.get(0).unwrap().split(',').map(|x| IdRange::new(x)).collect::<Vec<IdRange>>();
        assert_eq!(1227775554, calculate_part_1(&ranges));
    }

    #[test]
    fn test_part_2() {
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_2.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let ranges = input.get(0).unwrap().split(',').map(|x| IdRange::new(x)).collect::<Vec<IdRange>>();
        assert_eq!(4174379265, calculate_part_2(&ranges));
    }

}
