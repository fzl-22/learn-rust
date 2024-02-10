// Conditional operators
// <
// >
// <=
// >=
// !=
//
// Logical operators
// &&
// ||
// !
//
// Precedence of logical operators
// () -> ! -> && -> ||

fn main() {
    // Conditions
    
    // Branching
    let cond1 = 2 < 3;
    let cond2 = (2 as f64) <= 2.2; // must be the same type
    let cond3 = false && cond2;
    let cond4 = false || cond2;
    let cond5 = !cond4;

    println!("{}", cond1);
    println!("{}", cond2);
    println!("{}", cond3);
    println!("{}", cond4);
    println!("{}", cond5);

    // Control Flows
    let food = "doughnut";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "doughnut" {
        println!("That sounds good!");
    } else if food == "breath" {
        println!("That sounds boring!");
    } else {
        println!("Oh, that's too bad!");
    }


    // Looping (while) 
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1
    }

    for n in 1..100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("names: {:?}", names);
}
