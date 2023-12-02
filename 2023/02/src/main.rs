use std::{fs, collections::HashMap};

struct Game<'a> {
  id: usize,
  rounds: Vec<HashMap<&'a str, usize>>
}

fn parse_game(raw: &str) -> Game {
  // honestly i could use a regex ideally
  // but i really dont want to right now
  let id = raw
    .split(":")
    .nth(0).unwrap()
    .split(" ").nth(1).unwrap()
    .parse::<usize>().unwrap();

  let mut new_game = Game {
    id,
    rounds: vec![]
  };

  let rounds_raw: Vec<&str> = raw
    .split(": ")
    .nth(1).unwrap()
    .split("; ").collect();

  for round in rounds_raw {
    let mut cubes: HashMap<&str, usize> = HashMap::new();
    let cubes_raw = round
    .split(", ");

    for cube in cubes_raw {
      let spl: Vec<&str> = cube.split(" ").collect();
      cubes.insert(
        spl.get(1).unwrap(), 
        spl.get(0).unwrap().parse::<usize>().unwrap()
      );
      // println!("inserted {} - {}", cube, round) 
    }
    new_game.rounds.push(cubes);
  }
  return new_game
}

fn main() {
  let str = fs::read_to_string("input.txt")
    .expect("couldnt read file");

  const COLORS: [&str; 3] = [
    "red",
    "green",
    "blue",
  ];

  const COLORS_LIMITS: [usize; 3] = [
    12,
    13,
    14
  ];

  let mut answer = 0;
  let mut powers = 0;

  for l in str.lines() {
    let g = parse_game(l);
    let is_valid = g.rounds.iter().all(|round| {
      for (ci, color) in COLORS.iter().enumerate() {
        let num = match round.get(color) {
          Some(c) => c,
          None => {
            continue;
          }
        };
        
        if num.gt(&COLORS_LIMITS[ci]) {
          return false;
        }
      }
      true
    });

    if is_valid {
      println!("Game {}: ✅", g.id);
      answer += g.id
    } else {
      println!("Game {}: ❌", g.id);
    }

    let mut minimum_cubes: HashMap<&str, usize> = HashMap::new();
    for i in g.rounds.iter() {
      for (color, amount) in i {
        if minimum_cubes.contains_key(color) {
          if amount > minimum_cubes.get(color).unwrap() {
            minimum_cubes.insert(color, *amount);
          }
        } else {
          minimum_cubes.insert(color, *amount);
        }
        
      }
    }

    powers += minimum_cubes.values()
      .fold(1, |acc, f| acc * f)
  }

  println!("answer is {}", answer);
  println!("powers answer is {}", powers);
}
