use std::fs;

pub fn day_6() {
  println!("\n\n--- Day 6 ---");

  let signal = fs::read_to_string("./src/day_6/input.txt").expect("ERROR : Unable to read file!");

  let first_sop = find_distinct_signal(&signal, 4).unwrap();
  let first_som = find_distinct_signal(&signal, 14).unwrap();

  println!("\nPart 1: {first_sop} characters before first start-of-packet");
  println!("\nPart 1: {first_som} characters before first start-of-message");
}

fn find_distinct_signal(signal: &String, length: usize) -> Option<usize> {
  'outer: for n in 0..=signal.len()-length {
    let slice = signal[n..n+length].as_bytes();

    // Check for match in rest of slice for each character
    for i in 1..slice.len() {
      if slice[i..].contains(&slice[i-1]) {
        continue 'outer
      }
    }

    // No match found, found distinct signal
    return Some(n + length)
  }
  return None
}