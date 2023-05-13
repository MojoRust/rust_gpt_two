pub fn run() {
// Part 3
    if !cfg!(feature = "syntax_usage") {
        return;
    }
    
        let grade = 91;

        let result = if grade >= 90 {
            "A"
        } else if grade >= 80 {
            if grade >= 85 {
                "B+"
            } else {
                "B"
            }
        } else if grade >= 70 {
            if grade >= 75 {
                "C+"
            } else {
                "C"
            } 
        } else {
            "F"
        };
    println!("Result {}", result);


// part Return enum
        

}