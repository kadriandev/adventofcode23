use std::usize;

use array2d::Array2D;

advent_of_code::solution!(3);

struct Schematic {
    data: Array2D<char>,
    symbols: Vec<Symbol>
}

struct Symbol {
    x: usize,
    y: usize,
    character: char
}

impl Schematic {
    fn new(input: &str) -> Schematic {
        let lines = input.lines();
        let mut rows: Vec<_> = vec![];
        for line in lines {
            rows.push(line.chars().collect::<Vec<char>>());
        }

        let data = Array2D::from_rows(&rows).unwrap();
        let mut symbols: Vec<Symbol> = vec![];
        for i in 0..data.num_rows() {
            for j in 0..data.num_columns() {
                if let Some(char) = data.get(i, j) {
                    if !char.eq_ignore_ascii_case(&'.') && !char.is_digit(10){
                        symbols.push(Symbol{x:i, y:j, character:*char});
                    }
                }

            }
        }

        Schematic { data, symbols }
    }
}

impl Symbol {
    fn parts(&self, data: &mut Array2D<char>) -> Option<Vec<u32>>{
        let mut parts: Vec<u32> = vec![];
        for i in 0..3 {
            for j in 0..3 {
                if let Some(test) = data.get(self.x - 1 + i, self.y - 1 + j) {
                    if !test.is_digit(10){
                        continue;
                    }
                    // Get whole part and push into vec
                    let mut num_idx = (self.x + i - 1, self.y + j - 1);
                    let mut end_idx = num_idx.1;
                    loop {
                        if num_idx.1 == 0 { break };
                        let char = data.get(num_idx.0, num_idx.1 - 1);
                        match char {
                            Some(c) => if c.is_digit(10) { num_idx.1 -= 1 } else {break},
                            None => break,
                        }
                    }

                    loop {
                        let char = data.get(num_idx.0, end_idx + 1);
                        match char {
                            Some(c) => if c.is_digit(10) { end_idx += 1 } else { break },
                            None => break,
                        }
                    }

                    let row = data.row_iter(num_idx.0).unwrap().collect::<String>();
                    if let Some(num) = row.get(num_idx.1..end_idx + 1) {
                        if let Ok(n) = num.parse::<u32>() {
                            parts.push(n);
                        }

                        for i in num_idx.1..end_idx + 1 {
                            data.set(num_idx.0, i, '.').ok();
                        }
                    }
                }
            }
        }
        Some(parts)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut schematic = Schematic::new(input);
    let mut parts_sum: u32 = 0;
    for sym in schematic.symbols.iter() {
        if let Some(parts) = sym.parts(&mut schematic.data) {
            parts_sum += parts.iter().sum::<u32>();
        }
    }
    Some(parts_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut schematic = Schematic::new(input);
    let mut gear_sum: u32 = 0;
    for sym in schematic.symbols.iter() {
        if sym.character.eq(&'*') {
            if let Some(parts) = sym.parts(&mut schematic.data)  {
                if parts.iter().len() == 2 {
                    let p = parts.to_vec();
                    gear_sum += p[0] * p[1];
                }
            }
        }
    }
    Some(gear_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
