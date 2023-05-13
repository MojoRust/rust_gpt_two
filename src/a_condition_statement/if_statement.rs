
pub fn run() {
// Syntax & Usage
    if !cfg!(feature = "if_statement") {
        return;
    } 
// Section x

    let x: i32 = 5;

        if x > 3 {
            println!("{} is greater than 3", x);
        } else {
            println!("{} is not greater than 3", x);
    }
// Section y
    let y: &str = "Hello";

        if y == "World" {
            println!("Hello, {}", y);
        } else {
            println!("{}, Rust!", y);
    }
// Section z

    let z: i32 = 10;

    if z < 5{
        println!("z is less than 5");
    } else if z == 5 {
        println!("z is equal to 5");
    } else {
        println!("z is more than 5");
    if !cfg!(feature = "if_statement") {
        return;
    } 
// Part 2
    let x = 7;

    let x_status = if x % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    println!("{}", x_status);

// Some
     let y: Option<i32> = Some(42);

     if let Some(value) = y {
        println!("y contain a value {}", value);
     } else {
        println!("y is none");
     }
// Err
     let z: Result<i32, &str> = Err("An error occured");

     if let Ok(result) = z {
        println!("Operation succeeded with result {}", result);
     } else {
        println!("Operation failed");
     }


     let number: Result<i32, &str> = Ok(40);

     // Chaining if let with else if let
     if let Ok(n) = number {
         println!("The number is {}", n); // Output: "The number is 42"
     } else if let Err(e) = number {
         println!("Error: {}", e);
     } else {
         println!("This will never be executed");
     }

    }
}

