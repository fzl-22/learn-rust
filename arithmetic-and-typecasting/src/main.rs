use std::io;

fn main() {
    // let x: f64 = 255.0;
    // let y: f64 = 10.0;

    let _x = 255.0f32; // typecast f64 yo f32
                       // or let x = 255.0_f32
    let _y = 10.0;

    let x = 127_000 as i64;
    let y = 10_i32;

    // can't do the addition if the operand is not the same
    // integer overflow will occure if there is not enough bits
    // to store the result of an operation.
    //
    // Recommended to cast from smaller type to larger type
    // to reduce the potential of overflow.
    let z = x / (y as i64);
    println!("{}", z);

    // Example of overflow
    let x = (i32::MAX as i64) + 1;
    println!("x = {}", x);

    let y = 10_i32;
    println!("y = {}", y);

    // z variable is overflowed
    let z = (x as i32) / y;
    println!("z = {}", z);

    // Example with user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("input: {}", int_input + 2);
}
