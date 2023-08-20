fn to_wareki_str(seireki: u32) -> String {
  let showa = 1926..=1988;
  let heisei = 1989..=2018;
  let reiwa = 2019..;

  if showa.contains(&seireki) {
    let wareki = seireki - showa.start() + 1;
    if wareki == 1 {
      return format!("昭和元年");
    }
    return format!("昭和{}", wareki);
  }
  if heisei.contains(&seireki) {
    let wareki = seireki - heisei.start() + 1;
    if wareki == 1 {
      return format!("平成元年");
    }
    return format!("平成{}", wareki);
  }
  if reiwa.contains(&seireki) {
    let wareki = seireki - reiwa.start + 1;
    if wareki == 1 {
      return format!("令和元年");
    }
    return format!("令和{}", wareki);
  }
  String::from("")
}

fn main() {
  let s = (1926..=2026)
    .map(|y| {
      format!("西暦{}年 = {}", y, to_wareki_str(y))
    })
    .collect::<Vec<String>>()
    .join("\n");
  println!("{}", s);
}
