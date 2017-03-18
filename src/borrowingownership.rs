pub fn ownership() {
    //the variable v is on the stack - the data it refers to is on the heap
    let v = vec![1,2,3];
    //a pointer to the same vector data looks like this
    let v2 = v;
    //this op actually invalidates v because of Rust's ownership props
    //same thing will happen if you pass v to a closure
    // println!("{:?}", v);
    //OK for primitives because the vals are not stored on the heap
    let u = 1;
    let u2 = u;
    println!("u2 = {}", u2);
    //to get similar behavior for primitives, Box::new(value) will allocate space on the heap for it
    //let u = Box::new(1);

    //in closures, returning the value passed in returns the ownership
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("x[0] = {}", x[0]);
        x
    };
    //line 19 effectively returns ownership along with the val - but borrowing makes this easier
}

pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("v[0] = {}", x[0]);
    };

    let v = vec![1,2,3];
    print_vector(&v); //ampersand means that the argument is borrowed - not owned
    println!("v[0] = {}", v[0]); //this is totally cool

    //borrowing mutability must match mutability of the variable
    let mut a = 40;
    { //without these braces to define scope, b never releases the variable so the println won't work
        let b = &mut a;
        *b += 2;
    }

    println!("a = {}", a);
}
