fn main() {
    // -------------------------
    // *** variable bindings ***
    // -------------------------

    // Type and inference
    let a = 1;                                  // type i32 is infered
    let b: i32 = 2;                             // type i32 is explicitly declared

    let (c, d) = (3, 4);                        // variable bindings of multiple vars at the same time

    let e: i32;
    //println!("The value of f is: {}", f);     // this would give an error at compile time, trying to use an uninitialized variable

    // Mutability and shadowing
    //a = 100;                                  // this is not allowed and would give an error at compile time

    let mut f = 5;                              // make variable mutable
    f = 6;                                      // now I can assign a new value (of the same type!) to this variable
    let f = "Look, I am text now";              // shadowing, and changing type!

    // scopes and binding
    let g: i32 = 7;
    println!("The value of g is: {}", g);       // prints "The value of g is: 7"

    {
        let g = 'G';                            // shadowing within the block only
        println!("The value of g is: {}", g);   // prints "The value of g is: G"

        let h = 8;                              // h is not accessible outside of the scope of this block
        println!("I can access both g [{}] and h [{}]!", g, h);
    }
    println!("The value of g is: {}", g);       // prints "The value of g is: 7"

    //let j = (let i = 5);                      // this is not allowed, in Rust assignment cannot be expressions
    let mut i = 5;
    let j = (i = 6);                            // the value of an assignment is an empty tuple: so j has the value (), not 6

    let k = sum(i, 3);
    println!("The value of k is: {}", k);

    let l = always_return_one(999999);
    println!("The value of l is: {}", l);

    //let m: String = diverges();                // a diverges type can be assigned to any type, in this example a String
                                                 // its output would kill the program, something like:
                                                 // thread 'main' panicked at 'This function never returns!', src/main.rs:62
                                                 // note: Run with `RUST_BACKTRACE=1` for a backtrace.

    let n: fn(i32, i32) -> i32 = sum;                 // this is a function pointer (without inference)
    // let n = sum;                                   // this would work too (using inference)
    println!("The value of n(k, l) is: {}", n(k, l)); // prints "The value of n(k, l) is: 10"

    // -----------------
    // *** types ***
    // -----------------

    // -- Booleans
    let a_boolean = true;                         // with inference
    let another_boolean: bool = false;            // without inference

    // -- Chars
    let this_is_a_char = 'x';                    // delimited by a single quote/tick
    let two_hearts_char = '💕';                   // any unicode char (all chars are 4 bytes, not just 1)

    // -- Numeric types
    let an_integer = 42;                         // default with inference (type i32 - The 32-bit signed integer type)
    let a_float = 1.0;                           // default with inference (type f64 - The 64-bit floating point type)

    // Numeric types are a combo of sign (signed/unsigned), size (fixed/variable), and integerness (integer/floating)
    // i8     - signed,    8-bit, integer
    // i16    - signed,   16-bit, integer
    // i32    - signed,   32-bit, integer
    // i64    - signed,   64-bit, integer
    // u8     - unsigned,  8-bit, integer
    // u16    - unsigned, 16-bit, integer
    // u32    - unsigned, 32-bit, integer
    // u64    - unsigned, 64-bit, integer
    // isize  - signed,   variable size, integer
    // usize  - unsigned, variable size, integer
    // f32    - signed,   32-bit, floating
    // f64    - signed,   64-bit, floating

    // -- Arrays
    // arrays have type [T, N]
    let immutable_array = [5, 7];            // type [i32; 2]
    let mut mutable_array = [1.5, 2.3, 3.0]; // type [f64; 3]
 
    // arrays are 0-based
    // you access a given item with the "subscript notation"
    println!("The second item is: {}", immutable_array[1]);         // prints "The second item is: 7"

    let array_with_default_values = [0; 20];                        // type [i32; 20] - an array of 20 zeros
    println!("Array size is: {}", array_with_default_values.len()); // gets the array's length
}

// -----------------
// *** functions ***
// -----------------

fn sum(x: i32, y: i32) -> i32 {                 // types are mandatory in the function's signature (both args and return value)
    x + y                                       // no semicolon, it's a return value; with a semicolon, it would return ()
}

fn always_return_one(x: i32) -> i32 {
    return 1;                                   // explicit return keyword, semicolon required
    x                                           // never executed
}

fn diverges() -> ! {                            // defines the return type as "diverges" using the bang symbol
    panic!("This function never returns!");     // panic! is a macro
}

