fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9...12 => "a ton",
        _ if (x % 2 == 0) => "some",
        _ => "lots of"
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let mut point = (10,0);

    match point {
        (0,0) => println!("at origin"),
        (0,y) => println!("on x axis, y = {}", y),
        (ref mut x,0) => { //you can mutate the referenced val
            *x += 2;
            println!("on y axis, x = {}", x);
        },
        (x,y) => println!("x={}, y={}", x, y)
    }

    println!("Point.X={:?}", point.0);
}
