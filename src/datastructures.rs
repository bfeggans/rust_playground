use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn print_this_line(&self) {
        println!("The line starts at ({}, {}) and ends at ({}, {})",
            self.start.x, self.start.y, self.end.x, self.end.y);
    }

    fn average_of_points(&self) -> Point {
        let average_x = (self.start.x + self.end.x) / 2.0;
        let average_y = (self.start.y + self.end.y) / 2.0;

        Point { x: average_x, y: average_y }
    }
}

enum Color {
    Red,
    Yellow,
    Blue,
    Purple,
    Green,
    Rgb(u8, u8, u8),
    Rgba{red: u8, green: u8, blue: u8, alpha: f32}
}

pub fn enums() {
    let c:Color = Color::Rgba{red: 0, green: 0, blue: 0, alpha: 0.5};

    match c {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
        Color::Rgb(0,0,0)
            | Color::Rgba{red: 0, green: 0, blue: 0, alpha: _} => println!("black"),
        _ => println!("other color")
    }
}

pub fn structures() {
    let start = Point { x: 3.0, y: 4.0 };
    let end = Point { x: 9.5, y: 1.3 };
    let line = Line { start: start, end: end };
    let avg_point = line.average_of_points();

    line.print_this_line();
    println!("The average X point is {}, and the average Y point is {}.", avg_point.x, avg_point.y);
}

pub fn options() {
    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> =
        if y != 0.0 {
            Some(x/y)
        } else {
            None
        };

    //way to handle option return val
    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("Heyo - infinite")
    }

    //another way to handle option return val
    if let Some(z) = result {
        println!("z: {}", z);
    }
}

pub fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];
    a[0] = 10;

    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("debug a: {:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("a not equal to [1, 2, 3, 4, 5]");
    }

    let b = [1u8; 10];
            //^ Allows you to specify the type of initialized vals

    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let matrix:[[f32;3]; 2] =
    [
        [1.0, 2.0, 5.0],
        [0.5, 3.0, 4.0]
    ];

    println!("{:?}", matrix);
}

pub fn vectors() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    //usize is an unsigned number in the bit size of the machine
    let index:usize = 0;
    vector[index] = 20;

    println!("vector = {:?}", vector);
    println!("vector length = {:?}", vector.len());
    println!("vector[0] = {}", vector[index]);

    //for out of bounds exception cases. better than -> let blake = vector[6]; //causes panic
    //vector.get returns an Option
    match vector.get(6) {
        Some(x) => println!("vector[6] = {}", x),
        None => println!("No ELs, bruh.")
    }

    //looping over vector
    for x in &vector {
        println!("{}", x);
    }

    vector.push(804);

    //remove value from vector
    //let last_element = vector.pop(); //returns Option
    match vector.pop() {
        Some(x) => println!("{}", x),
        None => println!("Nah, homie.")
    }

    //iteration while popping
    while let Some(x) = vector.pop() {
        println!("x = {}", x);
    }
    println!("{:?}", vector);
}

pub fn slices() {
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn use_slice(slice: &mut [i32]) {
    slice[0] = 432;
    println!("first element={}, len={}", slice[0], slice.len());
}

pub fn strings() {
    //two different string types in Rust
    //&str = string slice; static means that the string is statically allocated
    let s:&'static str = "sup, yo?";

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First char is {}", first_char);
    }

    //String = heap allocated utf8 sequence
    //much more flexible than &str
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) { //TODO: Review this
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    //concatenation
    let mut blake = String::new();

    blake += "blake ";
    blake += "feggans";

    println!("{}", blake);

    //string from string slice
    let from_slice = String::from("Blake Feggans");
    println!("{}", from_slice);
    let mut from_slice_2 = "Hello, world!".to_string();
    from_slice_2.remove(6);
    from_slice_2.push_str("??");
    from_slice_2 = from_slice_2.replace("Hello", "Sup");
    println!("{}", from_slice_2);
}


//tuples are kinda like arrays but you can use non-matching types
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sum_product = sum_and_product(x, y);

    println!("{:?}", sum_product);
    println!("{0} + {1} = {2}, {0} + {1} = {3}", x, y, sum_product.0, sum_product.1);

    //destructuring
    //the structures must match
    let (a, b) = sum_product;
    println!("{}, {}", a, b);
    let sp2 = sum_and_product(10, 12);
    let combined = (sum_product, sp2);
    println!("{:?}", combined);
    println!("last item is {}", (combined.1).1);
    //destructuring works the same
    let ((c,d),(e,f)) = combined;
    println!("first tuple is ({}, {}), second tuple is ({}, {})", c, d, e, f);
    //to make a single element tuple use a comma
    let single_tuple = (43,);
    println!("{:?}", single_tuple);
}
