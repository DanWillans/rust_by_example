pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

// Compiling a library
// $ rustc --crate-type=lib rary.rs
// $ ls lib*
// library.rlib


// Using an external crate

// // extern crate rary; // May be required for Rust 2015 edition or earlier

// fn main() {
//     rary::public_function();

//     // Error! `private_function` is private
//     //rary::private_function();

//     rary::indirect_access();
// }


// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
// called rary's `public_function()`
// called rary's `indirect_access()`, that
// > called rary's `private_function()`