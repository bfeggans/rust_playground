#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;
extern crate phrases;

mod fundamentals;
mod stackheap;
mod controlflow;
mod datastructures;
mod patternmatching;
mod functions;
mod borrowingownership;
mod cratestuff;

use std::time::SystemTime;
use rand::Rng;
use phrases::greetings::french as fr;

const GLOBAL:&'static str = "Blake"; //no fixed address

fn main() {
    // println!("{}", GLOBAL);
    let mut now = SystemTime::now();
    let n = 50;

    let fib_i = fibonacci_iterative(n);
    println!("fib_i={}", fib_i);
    print_elapsed_secs(&now);

    now = SystemTime::now();
    let fib_r = fibonacci_recursive(n);
    println!("fib_r={}", fib_r);
    print_elapsed_secs(&now);
}

fn print_elapsed_secs(now: &SystemTime) {
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{} secs", elapsed.as_secs());
        },
        Err(e) => {
            println!("Some error: {:?}", e);
        }
    }
}

fn crate_usage() {
    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("French: {}, {}",
        fr::hello(),
        fr::goodbye()
    );
}

fn fibonacci_recursive(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    let mut counter = n;

    if counter < 2 { return counter }
    while counter > 1 {
        counter -= 1;
        c = a + b;
        a = b;
        b = c;
    }
    c
}
