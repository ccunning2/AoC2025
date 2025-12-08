use std::collections::HashMap;
use std::usize;
use aoc_utils::read_input;

fn main() {

    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../input/3.txt");
    let input = read_input(input_path.to_str().unwrap()).unwrap();
    println!("Part 1: {}", calculate_part_1(&input));
    println!("Part 2: {}", calculate_part_2(&input));
}




fn calculate_part_1(input: &Vec<String>) ->i64 {
     input.iter().map(|x| BatteryBank::new_part_1(x).jolts).sum()
    // banks.iter().map(|x| x.jolts).sum()
}

fn calculate_part_2(input: &Vec<String>) -> i64 {
 input.iter().map(|x| BatteryBank::new_part_2(x).jolts).sum()
}

#[derive(Debug)]
struct BatteryBank {
    bank: String,
    battery1: String,
    battery2: String,
    battery3: String,
    battery4: String,
    battery5: String,
    battery6: String,
    battery7: String,
    battery8: String,
    battery9: String,
    battery10: String,
    battery11: String,
    battery12: String,
    jolts: i64
}

impl BatteryBank {
    fn new_part_1(input: &str) -> Self {
        //Want maximum first digit followed by maximum second digit
        //Will always be max value unless last char
        let mut max = (usize::MAX, -1);
        let mut second_max = (0, -1);
        input.chars().enumerate().for_each(|(i, c)| {
            let val = i32::from_str_radix(&c.to_string(), 10).unwrap();
            if val > max.1 && i != (input.len() - 1) {
                max = (i, val);
                //reset second max
                second_max = (0, -1);
            } else if val > second_max.1 && i > max.0 {
                second_max = (i, val);
            }
        });
        Self {
            bank: input.to_string(),
            battery1: max.1.to_string(),
            battery2: second_max.1.to_string(),
            battery3: "".to_string(),
            battery4: "".to_string(),
            battery5: "".to_string(),
            battery6: "".to_string(),
            battery7: "".to_string(),
            battery8: "".to_string(),
            battery9: "".to_string(),
            battery10: "".to_string(),
            battery11: "".to_string(),
            battery12: "".to_string(),
            jolts: (max.1.to_string() + &second_max.1.to_string()).parse::<i64>().unwrap()
        }
    }

    fn new_part_2(input: &str) -> Self {
        let mut batteries: HashMap<usize,String> = HashMap::new();
        let mut start_pos = 0;
        for i in 1..=12 {
            let slice = &input[start_pos..input.len() - (12 - i)];
           let (new_pos, val) = Self::max_in_slice(slice, start_pos);
            start_pos = new_pos;
            batteries.insert(i, val.to_string());
        }
        let battery1 = batteries.get(&(1usize)).unwrap().to_string();
        let battery2 = batteries.get(&(2usize)).unwrap().to_string();
        let battery3 = batteries.get(&(3usize)).unwrap().to_string();
        let battery4 = batteries.get(&(4usize)).unwrap().to_string();
        let battery5 = batteries.get(&(5usize)).unwrap().to_string();
        let battery6 = batteries.get(&(6usize)).unwrap().to_string();
        let battery7 = batteries.get(&(7usize)).unwrap().to_string();
        let battery8 = batteries.get(&(8usize)).unwrap().to_string();
        let battery9 = batteries.get(&(9usize)).unwrap().to_string();
        let battery10 = batteries.get(&(10usize)).unwrap().to_string();
        let battery11 = batteries.get(&(11usize)).unwrap().to_string();
        let battery12 = batteries.get(&(12usize)).unwrap().to_string();
        let jolt_string = format!("{}{}{}{}{}{}{}{}{}{}{}{}",
        battery1, battery2, battery3, battery4,battery5,battery6,battery7,battery8,battery9,battery10,battery11,battery12);

        Self {
            bank: input.to_string(),
            battery1,
            battery2,
            battery3,
            battery4,
            battery5,
            battery6,
            battery7,
            battery8,
            battery9,
            battery10,
            battery11,
            battery12,
            jolts: jolt_string.parse::<i64>().unwrap()
        }
    }

    fn max_in_slice(slice:&str, start_pos:usize) -> (usize, i32) { //returns index, value
        let mut max_val = (usize::MAX, i32::MIN);
        slice.chars().enumerate().for_each(|(i, c)| {
            let num = i32::from_str_radix(&c.to_string(), 10).unwrap();
            if num > max_val.1 {
                max_val = (i + 1 + start_pos, num);
            }
        });
        max_val
    }

}





#[cfg(test)]
mod test {
    use super::*;
    use aoc_utils::read_input;

    #[test]
    fn test_part_1() {

        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_3.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
        // let banks = input.iter().map(|x| BatteryBank::new_part_1(x)).collect::<Vec<BatteryBank>>();
        // assert_eq!(1227775554, calculate_part_1(&ranges));
       let ans:i64 =  calculate_part_1(&input);
        //357
        assert_eq!(357, ans);
    }

    #[test]
    fn test_part_2() {

        let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../input/test_3.txt");
        let input = read_input(input_path.to_str().unwrap()).unwrap();
         assert_eq!(3121910778619, calculate_part_2(&input));
    }

}