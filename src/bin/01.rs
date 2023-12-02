use std::u32;

use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut calibration: u32 = 0;

    for line in lines {
        let mut value = String::new();
        let chars = line.chars();
        for char in chars {
            if char.is_digit(10) {
                if value.len() > 1 {
                    value.pop();
                }
                value.push(char);
            }
        }

        if value.len() == 1 {
            value.push(value.chars().next().unwrap())
        }

        if let Ok(val) = value.parse::<u32>() {
            calibration += val;
        }
    }
    Some(calibration)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let lines = input.lines();
    let mut calibration: u32 = 0;

    let re = Regex::new(r"\d{1}|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_reversed = Regex::new(r"\d{1}|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    for line in lines {
        if let Some(first) = re.find(line).map(|c| c.as_str()) {
            print!("First: {}", first);

            if let Ok(val) = first.parse::<u32>() {
                calibration += val * 10;
            } else {
                let val = numbers.iter().position(|&r| r == first).unwrap() as u32 + 1;
                calibration += val * 10;
            }
        }

        if let Some(last) = re_reversed
            .find(line.chars().rev().collect::<String>().as_str())
            .map(|c| c.as_str().chars().rev().collect::<String>())
        {
            println!("Last: {}", last);

            if let Ok(val) = last.as_str().parse::<u32>() {
                calibration += val;
            } else {
                let val = numbers.iter().position(|&r| r == last).unwrap() as u32 + 1;
                calibration += val;
            }
        }
    }

    Some(calibration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142)
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.unwrap(), 281);
    }
}
