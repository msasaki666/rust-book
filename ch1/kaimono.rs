fn main() {
  let pc_price = 98000.0;
  let a_res = (pc_price * 0.8) + 1200.0;
  let b_res = pc_price * 0.9;

  println!("Aの価格: {}", a_res);
  println!("Bの価格: {}", b_res);
  if a_res > b_res {
    println!("bの方がやすい");
  } else {
    println!("aの方がやすい");
  }
}
