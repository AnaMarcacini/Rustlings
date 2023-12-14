fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}



// Constants
// Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

// First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. We’ll cover types and type annotations in the next section, “Data Types”, so don’t worry about the details right now. Just know that you must always annotate the type.



// Here’s an example of a constant declaration:

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// The constant’s name is THREE_HOURS_IN_SECONDS and its value is set to the result of multiplying 60 (the number of seconds in a minute) by 60 (the number of minutes in an hour) by 3 (the number of hours we want to count in this program). Rust’s naming convention for constants is to use all uppercase with underscores between words. The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. See the Rust Reference’s section on constant evaluation for more information on what operations can be used when declaring constants.