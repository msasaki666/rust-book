fn main() {
  for y in 1..=9 {
    for x in 1..=9 {
      // スペースを使って右寄せ３文字に
      print!("{:3},", y * x)
    }
    println!("")
  }
}
