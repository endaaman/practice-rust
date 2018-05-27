fn fizz_buzz(count: i64) {
    // let mut i = 0;
    for i in 1..count {
        if i % 15 == 0 {
            println!("FizzBuzz");
            continue;
        }
        if i % 5 == 0 {
            println!("Buzz");
            continue;
        }
        if i % 3 == 0 {
            println!("Fizz");
            continue;
        }
        println!("{}", i);
    }
}

