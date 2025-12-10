use std::collections::HashSet;
use aoc_utils::read_input;

fn main() {

    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/4.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    let mut mapping = map_paper(&input);
    println!("Part1 : {}", calculate_part_1(&mapping));
    println!("Part2: {}", calculate_part_2(&mut mapping));
}

fn calculate_part_1( mapping:&HashSet<(i32,i32)>) -> i32 {
    let mut count = 0;
    let mut paper_count = 0;
    for (x,y) in mapping {
            count += look_up_left(*x,*y, mapping);
            count += look_up(*x,*y, mapping);
            count += look_up_right(*x,*y, mapping);
            count += look_left(*x,*y, mapping);
            count += look_right(*x,*y, mapping);
            count += look_down_left(*x,*y, mapping);
            count += look_down(*x,*y, mapping);
            count += look_down_right(*x,*y, mapping);
            if count < 4 {
                paper_count += 1;
            }
            count = 0;
        }

    paper_count
}

fn calculate_part_2( mapping:&mut HashSet<(i32,i32)>) -> i32 {
    let mut count = 0;
    let mut paper_count = 0;
    let mut to_remove = HashSet::new();
    loop {
        for (x, y) in mapping.iter() {
            count += look_up_left(*x, *y, mapping);
            count += look_up(*x, *y, mapping);
            count += look_up_right(*x, *y, mapping);
            count += look_left(*x, *y, mapping);
            count += look_right(*x, *y, mapping);
            count += look_down_left(*x, *y, mapping);
            count += look_down(*x, *y, mapping);
            count += look_down_right(*x, *y, mapping);
            if count < 4 {
                paper_count += 1;
                to_remove.insert((*x, *y));
            }
            count = 0;
        }
        //Cleanup and check for break
        if to_remove.is_empty() {
            break;
        }
        for (x, y) in &to_remove {
            mapping.remove(&(*x, *y));
        }
        to_remove.clear();
    }
    paper_count
}

fn look_up(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x, y-1)) {
        return 1;
    }
    0
}

fn look_up_left(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x-1, y-1)) {
        return 1;
    }
    0
}

fn look_up_right(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x+1, y-1)) {
        return 1;
    }
    0
}

fn look_down(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x, y+1)) {
        return 1;
    }
    0
}

fn look_down_left(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x-1, y+1)) {
        return 1;
    }
    0
}

fn look_down_right(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x+1, y+1)) {
        return 1;
    }
    0
}

fn look_left(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x-1, y)) {
        return 1;
    }
    0
}

fn look_right(x:i32, y: i32, paper:&HashSet<(i32, i32)>) -> i32 {
    if paper.contains(&(x+1, y)) {
        return 1;
    }
    0
}


fn map_paper(data: &Vec<String>) -> HashSet<(i32, i32)> {
    let mut results = HashSet::new();
    data.iter().enumerate().for_each(|(y,s)| {
        s.chars().enumerate().for_each(|(x,c)| {
            if c == '@' {
                results.insert((x as i32, y as i32));
            }
        })
    });
    results
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
            .join("../input/test_4.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let mapping = map_paper(&input);
        assert_eq!(13, calculate_part_1(&mapping));
    }

    #[test]
    fn test_part_2() {
        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_4.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        let mut mapping = map_paper(&input);
        assert_eq!(43, calculate_part_2(&mut mapping));
    }


}
