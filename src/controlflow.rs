#![allow(dead_code)]

pub fn if_statement() {
    let temp = 70;

    if temp > 40 {
        println!("In C, it's hot.");
    } else if temp < 40 {
        println!("In F, it's cold!");
    } else {
        println!("It's 40 degrees.")
    }

    let clothing = if temp > 60 {"No coat."} else {"Get a coat."};
    println!("{}", clothing);
}

pub fn while_stuff() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        if x == 256 { continue; }
        println!("{}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y={}", y);

        if y > 500 { break; }
    }
}

pub fn for_loop_stuff() {
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x={}", x);
    }

    for (position, y) in (30..41).enumerate() {
        println!("Index={}, Value={}", position, y);
    }
}

pub fn match_stuff() {
    let country_code = 467;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        _ => "unknown"
    };

    println!("Country with code {} is {}", country_code, country);
}
