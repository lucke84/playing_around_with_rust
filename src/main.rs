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
    let j = (i = 6);                            // the value of an assignment is an empty tuple: so i has the value (), not 6

    let k = sum(i, 3);
    println!("The value of k is: {}", k);

    let l = always_return_one(999999);
    println!("The value of l is: {}", l);

    //let m: String = diverges();                // a diverges type can be assigned to any type, in this example a String
                                                 // its output would kill the program, something like:
                                                 // thread 'main' panicked at 'This function never returns!', src/main.rs:62
                                                 // note: Run with `RUST_BACKTRACE=1` for a backtrace.
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

