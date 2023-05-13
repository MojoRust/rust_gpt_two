fn find_username(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

pub fn run() {
    // ----------------------------------------------------------------

    if !cfg!(feature = "if_let_syntax") {
        return;
    }

    // ----------------------------------------------------------------

    let user = find_username(1);
    match user {
        Some(name) => println!("Username found: {}", name),
        None => println!("Username not found"),
    }

    // ----------------------------------------------------------------

    let result: Result<f64, String> = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("Result of division {}", value),
        Err(e) => println!("Error :{}", e),
    }

    // ----------------------------------------------------------------

    let a_value: Option<i32> = Some(40);
    // let a_value: Option<i32> = None;

    if let Some(41) = a_value {
        println!("It's same value");
    } else if let Some(b_value) = a_value {
        println!("it's difference value, must be: {}", b_value);
    } else {
        println!("No value found");
    }
}
