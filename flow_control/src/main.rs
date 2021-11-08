fn main() {
  // For Loop
  for_loop_ownership();
  for_loop_read_only();
  for_loop_read_write();
  anonymous_loops();

  // If else
  if_else();
  with_match();
  match_with_multiple_values();
}

fn match_with_multiple_values() {
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for item in &haystack {
    let result = match item {
      42 | 132 => "hit!",
      _ => "miss",
    };

    if result == "hit!" {
      println!("{}: {}", item, result);
    }
  }
}

fn with_match() {
  let n = 123456;
  let description = match is_even(n) {
    true => "even",
    false => "odd",
  };
  println!("{} is {}", n, description);
}

fn if_else() {
  let n = 123456;

  let description = if is_even(n) { "even" } else { "odd" };

  println!("{} is {}", n, description);
}

fn for_loop_ownership() {
  let collection = [1, 2];
  // for i in IntoIterator::into_iter(collection) {}
  for i in collection {
    println!("Owner {}", i);
  }
}

fn for_loop_read_only() {
  let collection = [10, 20];
  // for i in collection.iter() {}
  for i in &collection {
    println!("Read-Only {}", i);
  }
}

fn for_loop_read_write() {
  let mut collection = [100, 200];

  // for i in collection.iter_mut() {}
  for i in &mut collection {
    *i += 1;
    println!("Read-Write {}", i);
  }
}

fn anonymous_loops() {
  // When local variable is not used, by convention, you'll use an
  // underscore(_).
  for _ in 0..2 {
    println!("Hello, Rust");
  }
}

// aux function that returs true if `n` is even else false
fn is_even(n: i32) -> bool {
  n % 2 == 0
}
