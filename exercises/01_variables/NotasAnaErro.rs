fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // let x = 6; // resolve
    x = 6;
    println!("The value of x is: {x}");
}




// --> NotasAnaErro.rs:4:5
// |
// 2 |     let x = 5;
// |         -
// |         |
// |         first assignment to `x`
// |         help: consider making this binding mutable: `mut x`
// 3 |     println!("The value of x is: {x}");
// 4 |     x = 6;
// |     ^^^^^ cannot assign twice to immutable variable

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0384`.