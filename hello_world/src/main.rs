fn main() {
  // Function call
  greet_world();
  // Function call with args
  greet_name("John Doe");
}

fn greet_name(name: &str) {
  // The exclamation mark indicates the use of macro.
  println!("Hello, {}!", name);
}

fn greet_world() {
  println!("Hello, World!");
  // For assignment uses let keyword.
  let germany = "Grüß Gott!";
  // Unicode supported.
  let japan = "ハロー・ワールド";
  // Arrays literals uses square brackets.
  let regions = [germany, japan];
  // iter() returns an iterator.
  for region in regions.iter() {
    // The ampersand "borrows" `region` for read-only access.
    println!("{}", &region);
  }
}
