use crate::utils;

use std::collections::HashMap;

pub fn run() {
  let changes = utils::request::get("https://adventofcode.com/2018/day/1/input").unwrap();

  // Part 1
  let mut frequency = 0;
  for change in &changes {
    frequency += change.parse::<i32>().unwrap();
  }
  println!("Part 1: {}", frequency);

  // Part 2
  let mut frequency_map: HashMap<i32, i32> = HashMap::new();
  let mut frequency = 0;
  let mut found = false;

  while !found {
    for change in &changes {
      let count: i32 = *frequency_map.get(&frequency).unwrap_or(&1);
      if count == 2 {
        found = true;
        break;
      }

      frequency_map.insert(frequency, count + 1);

      let change = change.parse::<i32>().unwrap();
      frequency += change;
    }
  }
  println!("Part 2: {}", frequency);
}
