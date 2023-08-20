fn main() {
  for i in 1..=50 {
    if i % 3 == 0 {
      println!("A");
      continue;
    }
    if i / 10 == 3 {
      println!("A");
      continue;
    }
    if i % 10 == 3 {
      println!("A");
      continue;
    }

    println!("{}", i);
  }
}
