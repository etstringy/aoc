use std::fs;
use std::collections::HashMap;

fn check_hashmap_substr(map: &HashMap<&str, u32>, string: &str, index: usize) -> Option<u32> {
  for i in map.iter() {

    if string[index..].len() < i.0.len() {
      continue;
    }

    if string[index..index + i.0.len()].eq(i.0.to_owned()) {
      return Some(*i.1)
    }
  }
  None
}

fn main() {
  let mut word_map: HashMap<&str, u32> = HashMap::new();
  word_map.insert("one", 1);
  word_map.insert("two", 2);
  word_map.insert("three", 3);
  word_map.insert("four", 4);
  word_map.insert("five", 5);
  word_map.insert("six", 6);
  word_map.insert("seven", 7);
  word_map.insert("eight", 8);
  word_map.insert("nine", 9);

  let str = fs::read_to_string("input.txt")
    .expect("couldnt read file");

  
  let mut result: Vec<u32> = vec![];
  for l in str.lines() {
    let mut num = 00;

    // first number
    for (chi, ch) in l.chars().enumerate() {
      let word_num = check_hashmap_substr(&word_map, l, chi);
      if word_num.is_some() {
        num += word_num.unwrap() * 10;
        break;
      }
      if ch.is_numeric() {
        num += ch.to_digit(10).unwrap() * 10;
        break;
      }
    }

    for (chi, ch) in l.chars().rev().enumerate() {
      let word_num = check_hashmap_substr(&word_map, l,l.len() - (chi +1));
      if word_num.is_some() {
        num += word_num.unwrap();
        break;
      }
      if ch.is_numeric() {
        num += ch.to_digit(10).unwrap();
        break;
      }
    }

    result.push(num);
  }

  // let result_str: Vec<String> = result
  // .iter()
  // .map(|&f| f.to_string())
  // .collect();
  // fs::write("output.txt", result_str.join("\n"));

  // println!("vec: {}", result_str.join(", "));

  let total: u32 = result
  .iter()
  .sum();

  println!("total: {}", total);

}
