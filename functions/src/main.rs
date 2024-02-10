fn main() {
    // let x = 20; // this is a statement
    // this is statement, but the inside is a expression
    let number = {
        let x = 3;
        x + 1
    };

    println!("Number: {}", number);

    // function call is an expression
    let result = add_numbers(10, 20);

    println!("{}", result);
}

// function declaration is also a statement
fn add_numbers(x: i32, y: i32) -> i32 {
    // println!("The sum is: {}", x + y);
    // x + y // don't add semicolon
    // or use return
    // return x + y;

    let result = x + y;

    if result > 10 {
        return result - 10;
    }

    return result;
}
