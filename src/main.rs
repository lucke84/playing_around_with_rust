fn main() {
    // -------------------------
    // *** variable bindings ***
    // -------------------------

    let a = 1;           // type i32 is infered
    let b: i32 = 2;      // type i32 is explicitly declared

    let (c, d) = (3, 4); // variable bindings (multiple vars at the same time)

    let mut e = 5;       // make variable mutable
    e = 6; 			     // without "mut" this would give a compile error

    let f: i32;     
    //println!("The value of f is: {}", f); // this will give an error at compile time
    
    let g: i32 = 7;
    println!("The value of g is: {}", g);

    {
    	let h = 8;
    	println!("I can access both {} and {}!", g, h); // h is not accessible outside of the scope of this block
    }
}
