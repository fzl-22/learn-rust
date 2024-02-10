fn main() {
    // by default, rust variable is immutable 
    // to change it, add mut keyword 
    // let mut x = 4;
    // or we can also redefine it
    let x = 4;
    println!("x is {}", x);
    // redefine immutable variable
    let x = 5;
    println!("x is {}", x);
    
    // scoping
    {
        // name shadowing
        // this variable shadows the outside x (outside scope)
        // let x = 2; 
        // it can also use the value of variable outside the scope
        let x = x - 2;
        println!("x is {}", x);
    }

    let x = x + 1;
    println!("x is {}", x);

    // if we redefined an immutable variable,
    // it is considered as a new variable.
    // So, we can assign it with different data type
    let x = "this is a string";
    println!("x is {}", x);

    // if from the start we define it as
    // -> let mut x = 4
    // we can't assign it with different data type
    // but we can still redefine it as immutable
    
    // const is a varible that the value and data type
    // can't be changed in the entirety of the program
    // once it's defined.
    // we also must define it type
    const SECONDS_IN_MINUTE: u32 = 60;

    println!("{}", SECONDS_IN_MINUTE);
}
