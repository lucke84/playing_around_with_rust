fn main() {
    // -------------------------
    // *** variable bindings ***
    // -------------------------

    let a = 1;                                  // type i32 is infered
    let b: i32 = 2;                             // type i32 is explicitly declared

    let (c, d) = (3, 4);                        // variable bindings (multiple vars at the same time)

    //a = 100;                                  // this is not allowed and would give an error at compile time

    let mut e = 5;                              // make variable mutable
    e = 6; 			                            // now I can assign a new value (of the same type!) to this variable
    let e = "Look, I am text now";              // shadowing, and changing type!

    let f: i32;     
    //println!("The value of f is: {}", f);     // this would give an error at compile time, trying to use an uninitialized variable
    
    let g: i32 = 7;
    println!("The value of g is: {}", g);       // prints "The value of g is: 7"

    {
        let g = 'G';                            // shadowing within the block only
        println!("The value of g is: {}", g);   // prints "The value of g is: G"    

    	let h = 8;                              // h is not accessible outside of the scope of this block
    	println!("I can access both g [{}] and h [{}]!", g, h); 
    }

    println!("The value of g is: {}", g);       // prints "The value of g is: 7"
}
