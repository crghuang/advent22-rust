use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Copied from: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn elf_with_most_calories(elves: &mut Vec<Elf>) { // &[Elf]) {
  let mut current_max = 0;
  let mut current_max_elf = 0;

  for elf in elves{
    if elf.total_calories > current_max {
      current_max = elf.total_calories;
      current_max_elf = elf.number;
    }
  }
  println!("\nElf {} has the most calories with {}.", current_max_elf, current_max);
}

pub fn top_three_elves(mut elves: &mut Vec<Elf>) {
  elves.sort_unstable_by_key(|item| (item.total_calories));
  let last_index = elves.len();
  println!("\nTop 3 caloric counts:\n  {}\n  {}\n  {}\nTotal = {}", elves[last_index-1].total_calories, elves[last_index-2].total_calories, elves[last_index-3].total_calories, elves[last_index-1].total_calories+elves[last_index-2].total_calories+elves[last_index-3].total_calories);
}

// Create new instance of 'Elf' for each grouping of unspaced lines
pub fn parse_food_list(file_path: String) -> Vec<Elf> {
  let mut number = 1;
  let mut elves = Vec::new();
  let mut i = Vec::new();
  let list = &mut i;

  if let Ok(lines) = read_lines(file_path) {
    for line in lines {
      if let Ok(caloric_value) = line {
        if !(caloric_value.is_empty()) {
          list.push(caloric_value.parse::<u32>().unwrap());
        } else {
          let mut new_elf = Elf{ number: number, total_calories: Default::default(), food_items: list.to_vec() };
          new_elf.calc_calorie_total();
          elves.push(new_elf);
          number += 1; // If multiple blank lines, this will increment multiple times
          list.clear();
        }
      }
    }
  }
  return elves
}

// Elf "class" = struct + impl
pub struct Elf {
  number: u32,
  total_calories: u32,
  food_items: Vec<u32>,
}

impl Elf {
  fn calc_calorie_total(&mut self) {
    self.total_calories = 0; // What's the best way to use struct default to set 'total_calories' field to 0?

    if self.food_items.len() != 0 {
      self.total_calories = self.food_items.iter().sum();
    }
  }
}

// struct Food {
//   calories: u32
// };
