fn main() {
    println!("Hello, world!");

    another_function();

    
    another_function2(5, 'h');
    // let x = (let y = 6);


    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");



    let x = retorna5();

    println!("The value of x is: {x}");


    let x = plus_one(5);

    println!("The value of x is: {x}");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    loop {
        println!("again!");
    }
}

fn another_function() {
    println!("Another function.");
}


fn another_function2(x: i32, unit_label: char ) {
    println!("The value of x is: {x}{unit_label}");
}

fn retorna5() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}