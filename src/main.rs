#![allow(dead_code)]
mod stackheap;
mod controlflow;
mod datastructures;
mod patternmatching;
use std::mem;

const GLOBAL:&'static str = "Blake"; //no fixed address

fn main() {
    //println!("{}", GLOBAL);
    //stackheap::stack_and_heap();
    //controlflow::match_stuff();
    //patternmatching::pattern_matching();
    datastructures::generics();

    // let fib = fibonacci(5);
}

// fn fibonacci(n: i64) -> i64 {
//     if n == 0 {
//         0
//     } else if n == 1 {
//         1
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 145;
        println!("inner b={}", b);

        let a = 733; //shadowing the outer a - actually a separate var
        println!("inner a={}", a);
    }

    println!("a={}", a);
    //println!("b={}", b); b not available in outer scope
}

fn operators() {
    let mut a = 2+3*4;
    println!("a={}", a);
    a = a+1;
    println!("a={}", a);
    a += 1;
    println!("a={}", a);
    a = i32::pow(a, 3);
    println!("a={}", a);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {} to the PI = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR, & AND, ^ EXCLUSIVE OR, ! NEG OR
    println!("1|2 = {}", c);

    //logical
    let pi_less_than_4 = std::f64::consts::PI < 4.0;
    // > < >= <= ==
    println!("PI less than 4? {}", pi_less_than_4);
}

fn fundamental_data_types() {
    //unsigned 8 bit integer ranges from 0 -> 255
    let a:u8 = 56;
    println!("a={}", a);

    let mut b:u8 = 12;
    println!("b={}", b);
    b = 65;
    println!("b={}", b);

    let c = 123456789; //32 bit signed int
    println!("c={}, size={}", c, mem::size_of_val(&c));

    //i8 u8 i16 u16 i32 u32 i64 u64

    let d:char = 'b';
    println!("d={}, size={}", d, mem::size_of_val(&d));

    let e:f64 = 2.56;
    println!("e={}, size={}", e, mem::size_of_val(&e));
}
