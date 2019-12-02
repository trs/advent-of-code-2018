use crate::utils;

use std::collections::HashMap;

fn get_duplicate_count(id: &String) -> (i32, i32) {
  let mut char_map: HashMap<char, i32> = HashMap::new();
  for c in id.chars() {
    let current = *char_map.get(&c).unwrap_or(&0);
    char_map.insert(c, current + 1);
  }

  let mut dups = (0, 0);
  for (_, &count) in char_map.iter().filter(|(_, &c)| c > 1) {
    if count == 3 {
      dups.1 = 1;
    } else if count == 2 {
      dups.0 = 1;
    }
  }

  dups
}

pub fn run() {
  let ids = utils::request::get("https://adventofcode.com/2018/day/2/input").unwrap();

  // Part 1
  let mut duplicates = (0, 0);
  for id in ids {
    println!("{}", id);
    let (twos, threes) = get_duplicate_count(&id);
    println!("{:#?}", duplicates);

    duplicates.0 += twos;
    duplicates.1 += threes;
  }

  let sum = duplicates.0 * duplicates.1;
  println!("Part 1: {}", sum);
}
