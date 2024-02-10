// Data types in Rust
// -> Scalar type   : int, bool, and etc.
// -> Compound type : array, tuple, and etc.
fn main() {
    // SCALAR TYPES
    let _int_type: i32 = 2; // signed int of 32 bits. (default)
                            // i8                           // signed int of 8 bits.
                            // i16                          // ...
                            // i32
                            // i64
                            // i128

    let _uint_type: u32 = 972; // unsigned int of 32 bits. (default)
                               // u8
                               // u16
                               // u32
                               // u64
                               // u128

    let _floating_point: f64 = 10.9; // double-precision floating point. (default)
                                     // f32
                                     // f64                          // single-precision floating point.

    let _bool_type: bool = false; // false
                                  // true                         // true

    let _char: char = 'a'; // single character type

    // COMPOUND TYPES

    // Tuple
    let tuple_type: (i32, bool, char) = (1, true, 's');
    let mut tuple_type_2: (i8, bool, char) = (1, true, 's'); // different tuple because of difference type
    println!("{} {} {}", tuple_type.0, tuple_type.1, tuple_type.2);

    // assign each element individually
    tuple_type_2.0 = 10;
    // assign whole tuple (have same size and all elements must be the same type)
    tuple_type_2 = (10, false, 'a');

    println!("{} {} {}", tuple_type_2.0, tuple_type_2.1, tuple_type_2.2);

    // Array
    // to define the type, use [type, num],
    // where type is array data type and num is the number of element.
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // unassigned array
    // let mut arr_empty: [i32; 5];
    // assigned, but different size
    // let mut arr_empty: [i32, 5] = [];
    // if array is immutable, it can't be assigned
    arr[4] = 3;

    println!("{}", arr[4]);
    // throws an error if uninitialized
    // println!("{}", arr_empty[3]);
    
    // If a variable y is assigned value from variable y, the type of x will follows y
    let x: u8 = 4;
    // let y: i32 = x; // can't be assigned
    println!("x = {}", x);
}

