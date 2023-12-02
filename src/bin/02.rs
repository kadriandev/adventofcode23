advent_of_code::solution!(2);


pub fn part_one(input: &str) -> Option<u32> {
    const REDMAX:u32 = 12;
    const GREENMAX:u32 = 13;
    const BLUEMAX:u32 = 14;

    let lines = input.lines();
    let mut valid_games: Vec<u32> = vec![];

    for line in lines {
        let mut game_validity = true;
        let mut game_entry = line.split(":");
        let game_num = game_entry.nth(0).unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
        let game = game_entry.nth(0).unwrap();

        let picks = game.split(";").into_iter();
        'game: for pick in picks {
            let lots = pick.split(",").into_iter();
            for lot in lots {
              let lot_arr: Vec<&str> = lot.split(" ").collect();
              match lot_arr[2] {
                  "red" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > REDMAX {
                          game_validity = false;
                          break 'game;
                      }
                  },
                  "green" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > GREENMAX {
                          game_validity = false;
                          break 'game;
                      }
                  },
                  "blue" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > BLUEMAX {
                          game_validity = false;
                          break 'game;
                      }
                  },
                  _ => ()
              }
            }
        }
        if game_validity {
            valid_games.push(game_num);
        }
    }
    Some(valid_games.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut combined_power: u32 = 0;

    for line in lines {
        let mut bag = (0,0,0);
        let game = line.split(":").last().unwrap();

        let picks = game.split(";").into_iter();
        for pick in picks {
            let lots = pick.split(",").into_iter();
            for lot in lots {
              let lot_arr: Vec<&str> = lot.split(" ").collect();
              match lot_arr[2] {
                  "red" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > bag.0 {
                          bag.0 = cubes;
                      }
                  },
                  "green" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > bag.1 {
                          bag.1 = cubes;
                      }
                  },
                  "blue" => if let Ok(cubes) = lot_arr[1].parse::<u32>() {
                      if cubes > bag.2 {
                          bag.2 = cubes;
                      }
                  },
                  _ => ()
              }
            }
        }

        combined_power += bag.0 * bag.1 * bag.2;
    }

    Some(combined_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
