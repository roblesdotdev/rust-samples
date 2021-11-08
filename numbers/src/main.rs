use std::convert::TryInto;

fn main() {
  // infers
  let twenty = 20;
  // type annotation
  let twenty_one: i32 = 21;
  // suffix
  let twenty_two = 22_i32;

  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  // underscores increase readibility
  let one_million: i64 = 1_000_000;
  // numbers have methods
  println!("{}", one_million.pow(2));

  // creates an array of numbers
  let forty_twos = [42.0, 42f32, 42.0_f32];

  // elements within array are indexed starting at 0
  println!("{:02}", forty_twos[0]);

  // comparing
  comparing_nums_with_as_convertion();
  comparing_nums_safety();
}

fn comparing_nums_with_as_convertion() {
  let a: i32 = 10;
  let b: u16 = 100;

  // Warning: use type casts will cause your program to
  // behave unexpected.
  if a < (b as i32) {
    println!("Ten is less than one hundred.");
  }
}

fn comparing_nums_safety() {
  let a: i32 = 10;
  let b: u16 = 100;

  // try_into() returns a Result type that provides access to the
  // conversion attempt.
  let b_ = b.try_into().unwrap();

  if a < b_ {
    println!("Ten is less than one hundred.");
  }
}
