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

fn find_common_id(ids: &Vec<String>) -> Option<String> {
  for i in 0..ids.len() {
    let compare_id = &ids[i];

    for current_id in &ids[(i + 1)..] {

      let mut diff_index: Vec<usize> = Vec::new();

      for i in 0..compare_id.len() {
        let compare_char = compare_id.chars().nth(i).unwrap();
        let current_char = current_id.chars().nth(i).unwrap();

        if compare_char != current_char {
          diff_index.push(i);
          if diff_index.len() == 2 {
            break;
          }
        }
      }

      if diff_index.len() == 1 {
        let wrong_index = diff_index[0];
        let common = format!("{}{}", &current_id[..wrong_index], &current_id[(wrong_index + 1)..]);
        return Some(common);
      }
    }
  }
  None
}

pub fn run() {
  let ids = utils::request::get("https://adventofcode.com/2018/day/2/input").unwrap();

  // Part 1
  let mut duplicates = (0, 0);
  for id in &ids {
    let (twos, threes) = get_duplicate_count(&id);

    duplicates.0 += twos;
    duplicates.1 += threes;
  }

  let sum = duplicates.0 * duplicates.1;
  println!("Part 1: {}", sum);

  // Part 2
  let common = find_common_id(&ids).unwrap();
  println!("Part 2: {}", common);
}
