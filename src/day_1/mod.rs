use crate::utility;

pub fn day_1() {
  let mut tmp = parse_food_list("./src/day_1/input.txt".to_string());
  let elves = &mut tmp;

  println!("\n\n--- Day 1 ---");
  elf_with_most_calories(elves);
  top_three_elves(elves);
}

// Find Elf with most calories
fn elf_with_most_calories(elves: &mut Vec<Elf>) { // &[Elf]) {
  let mut current_max = 0;
  let mut current_max_elf = 0;

  for elf in elves{
    if elf.total_calories > current_max {
      current_max = elf.total_calories;
      current_max_elf = elf.number;
    }
  }
  println!("\nPart 1: Elf {} has the most calories with {}.", current_max_elf, current_max);
}

// Find and display top three elves' calories
fn top_three_elves(elves: &mut Vec<Elf>) {
  // Sort vector of all Elf instances by total calories for each
  elves.sort_unstable_by_key(|item| (item.total_calories));

  let last_index = elves.len();

  println!("\nPart 2: Top 3 caloric counts:\n  {}\n  {}\n  {}\n\nTotal = {}",
    elves[last_index-1].total_calories,
    elves[last_index-2].total_calories,
    elves[last_index-3].total_calories,
    elves[last_index-1].total_calories+elves[last_index-2].total_calories+elves[last_index-3].total_calories);
}

// Create new instance of 'Elf' for each grouping of unspaced lines, return
// list of all elves in 'input.txt'
fn parse_food_list(file_path: String) -> Vec<Elf> {
  let mut number = 1;
  let mut elves = Vec::new();
  let mut i = Vec::new();
  let list = &mut i;

  if let Ok(lines) = utility::read_lines(file_path) {
    for line in lines {
      if let Ok(caloric_value) = line {
        if !(caloric_value.is_empty()) {
          // Push caloric value in line to temp array
          list.push(caloric_value.parse::<u32>().unwrap());
        } else {
          // Create new instance of Elf after blank line
          let mut new_elf = Elf{ number: number,
                                 total_calories: Default::default(),
                                 food_items: list.to_vec() };


          // Calculate, populate 'total_calories' field for Elf
          new_elf.calc_calorie_total();

          // Add Elf to list of all elves
          elves.push(new_elf);

          // Increment Elf 'number'
          number += 1; // If multiple blank lines, this will increment multiple times

          // Clear temp calories list for current Elf
          list.clear();
        }
      }
    }
  }
  return elves
}

// Elf "class" = struct + impl
struct Elf {
  number: u32,
  total_calories: u32,
  food_items: Vec<u32>,
}

impl Elf {
  fn calc_calorie_total(&mut self) {
    self.total_calories = 0; // What's the best way to use struct default to set 'total_calories' field to 0?

    // Sum all caloric values in vector
    if self.food_items.len() != 0 {
      self.total_calories = self.food_items.iter().sum();
    }
  }
}
