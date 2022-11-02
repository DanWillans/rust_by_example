// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

pub fn cfg() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}


// Some conditionals like target_os are implicitly provided by
// rustc, but custom conditionals must be passed to rustc using the --cfg flag.

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

// $ rustc --cfg some_condition custom.rs && ./custom
// condition met!
