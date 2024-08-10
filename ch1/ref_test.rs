fn main() {
  let mut v = 10;
  set_value(&mut v);
  println!("v={}", v);
}

fn set_value(num: &mut u32) {
  *num = 100;
}
