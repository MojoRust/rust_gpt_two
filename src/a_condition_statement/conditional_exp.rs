pub fn run() {

    if !cfg!(feature = "conditional_exp") {
        return;
    }

        let x: i32 = 5;

        let result: &str = if x > 3 {
            "x is greater than 3"
        } else {
            "x is not greater than 3"
        };

        println!("result: {}", result);

    // section y

        let y: i32 = 7;

        let description = if y < 5 { 
            "y is less than 5"
        } else if y == 5 {
            "y is equal to 5"
        } else {
            "y is greater than 5"
        };

        println!("description: {}", description);


}  
