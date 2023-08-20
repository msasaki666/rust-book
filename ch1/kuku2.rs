// 本のやり方
// fn main() {
//   for y in 1..=9 {
//     let s = (1..=9)
//     .map(|x| format!("{:3}", y * x))
//     .collect::<Vec<String>>().join(",");
//     println!("{}", s);
//   }
// }

// forなしバージョン
fn main() {
    let res = (1..=9)
        .map(|y| {
            (1..=9)
                .map(|x| format!("{:3}", y * x))
                .collect::<Vec<String>>()
                .join(",")
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", res);
}
