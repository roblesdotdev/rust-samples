fn main() {
  // Types can be inferred by the compiler...
  let a = 10;
  // or declarated explicitly.
  let b: i32 = 20;
  // Numeric types can include a type annotation in their literal form.
  let c = 30i32;
  // Numbers can include underscores, which are intended to increase
  // readability and have no functional impact.
  let d = 40_i32;
  let e = add(add(a, b), add(c, d));

  println!("The result is {}", e);
}

// Type declarations are required when defining functions.
fn add(i: i32, j: i32) -> i32 {
  // Functions returns the last expression's result.
  i + j
}
