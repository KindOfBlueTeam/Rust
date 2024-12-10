fn main() {
    // this is a comment in Rust
    /*
        A block comment
        to add various lines
    */

    // Printing and String Substitutions
    let some_obj: &str = "BAZ!"; // https://users.rust-lang.org/t/string-str-string-str/100611/4
    println!("Hello, world!"); // prints the newline
    print!("Foo, "); // prints, no new line
    println!("bar {}!", some_obj); // now with string substitution

    // Working with numbers
    // https://doc.rust-lang.org/book/ch03-02-data-types.html
    // i32 (32-bit) for speed ; i64 (4-bit) for precision ; Rust defaults to i32
    let unsigned_int: i32 = 42; // an unsigned int is always 0 or positive 
    let signed_int: i32 = -88;  // a signed int can be nagative or positive (and 0?)
    let number = 15; // declare object; Rust will assume this is an i32

    // Conditional testing and triggers
    if unsigned_int > 2 { 
        println!("{} is larger than 2!", unsigned_int); // action if true
    }
    if signed_int < 2 { 
        println!("{} is smaller than 2!", signed_int); // action if true
    }
    if number > 10 { // conditional check
        println!("{} is larger than 10!", number); // action if true
        }
    }



