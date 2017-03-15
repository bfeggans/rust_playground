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
