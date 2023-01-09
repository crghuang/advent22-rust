use std::fs;
use std::collections::VecDeque;

pub fn day_3() {
  println!("\n\n--- Day 3 ---");

  let rucksack_list = fs::read_to_string("./src/day_3/input.txt").expect("ERROR : Unable to read file!");

  let rucksacks = rucksack_list.split_whitespace();
  let mut priority_sum = 0;
  let mut badge_sum = 0;
  let mut group_rucksacks: VecDeque<&str> = VecDeque::with_capacity(3);

  for (i, rucksack) in rucksacks.enumerate() {
    let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);
    // Search for common letter in both compartments of each rucksack
    for letter in comp1.chars() {
      if comp2.contains(letter) {
        priority_sum += priority(letter);
        break;
      }
    }

    group_rucksacks.push_back(rucksack);
    if i > 2 { // Maintain VecDeque size 3
      group_rucksacks.pop_front();
    }
    // Look for common letter across three rucksacks in each Badge group
    if (i+1) % 3 == 0 {
      for letter in group_rucksacks.get(0).unwrap().chars() {
        if group_rucksacks.get(1).unwrap().contains(letter) && group_rucksacks.get(2).unwrap().contains(letter) {
          badge_sum += priority(letter);
          break;
        }
      }
    }
  }
  println!("\nPart 1: Priority sum = {priority_sum}");
  println!("\nPart 2: Badge sum = {badge_sum}");
}

fn priority(letter: char) -> u32 {
  match letter {
    'a' => 1,
    'b' => 2,
    'c' => 3,
    'd' => 4,
    'e' => 5,
    'f' => 6,
    'g' => 7,
    'h' => 8,
    'i' => 9,
    'j' => 10,
    'k' => 11,
    'l' => 12,
    'm' => 13,
    'n' => 14,
    'o' => 15,
    'p' => 16,
    'q' => 17,
    'r' => 18,
    's' => 19,
    't' => 20,
    'u' => 21,
    'v' => 22,
    'w' => 23,
    'x' => 24,
    'y' => 25,
    'z' => 26,
    'A' => 27,
    'B' => 28,
    'C' => 29,
    'D' => 30,
    'E' => 31,
    'F' => 32,
    'G' => 33,
    'H' => 34,
    'I' => 35,
    'J' => 36,
    'K' => 37,
    'L' => 38,
    'M' => 39,
    'N' => 40,
    'O' => 41,
    'P' => 42,
    'Q' => 43,
    'R' => 44,
    'S' => 45,
    'T' => 46,
    'U' => 47,
    'V' => 48,
    'W' => 49,
    'X' => 50,
    'Y' => 51,
    'Z' => 52,
    _ => panic!("Invalid character!"),
  }
}