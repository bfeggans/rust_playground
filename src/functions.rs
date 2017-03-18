pub fn funcs() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    println!("z={}", z);
    println!("{}", products(4, 6));

    methods();
    closures();
    higher_order();
    traits();
}

fn print_value(x: i32) {
    println!("x={}", x);
}

fn increase(x: &mut i32) {
    *x += 1
}

fn products(x: i32, y: i32) -> i32 {
    x * y
}

struct Blake {
    age: u8,
    last_name: String
}

impl Blake {
    fn get_age(&self) -> u8 {
        self.age
    }
}

fn methods() {
    let me = Blake { age: 29, last_name: "Feggans".to_string() };
    println!("I'm {} years old.", me.get_age());
}

fn closures() {
    let plus_one = |x:i32| -> i32 { x + 1 };

    println!("{}", plus_one(3));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut a = x;
            a += two;
            a
        };
        println!("{}", plus_two(3));
    }
    let borrow_two = &mut two;

    let plus_three = |x: &mut i32| *x += 3;
    let mut num = 12;
    plus_three(&mut num);
    println!("num = {}", num);
}

fn higher_order() {
    fn is_even(x: u32) -> bool {
        x % 2 == 0
    }

    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit { break; }
        else if is_even(isq) { sum += isq; }
    }

    println!("The sum is {}", sum);

    let sum2 =
        (0..).map(|x| x*x)
             .take_while(|&x| x < limit) //impl requires that you pass in var with &
             .filter(|x| is_even(*x))
             .fold(0, |sum, x| sum + x); //like arr.reduce in JS

    println!("The sum2 is {}", sum2);
}

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk.", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello.", self.name());
    }
}

fn traits() {
    let h:Human = Animal::create("John");
    h.talk();
}
