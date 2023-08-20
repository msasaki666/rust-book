fn main() {
    // 1..101でもOK
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
            continue;
        }
        if i % 3 == 0 {
            println!("Fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("Buzz");
            continue;
        }
        println!("{}", i);
    }
}
