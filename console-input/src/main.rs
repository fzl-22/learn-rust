use std::io;

fn main() {
    // initalized mutable String input
    let mut input = String::new();

    // read line from stdin and modify input by reference
    // expect "failed to read line" output if any error occured
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
