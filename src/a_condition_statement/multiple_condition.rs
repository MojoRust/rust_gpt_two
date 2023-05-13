pub fn run() {
    if !cfg!(feature = "multiple_condition"){
        return;
    }

    let fruit = "hello";

    if fruit == "apple" {
        println!("It's an apple");
    } else if fruit == "banana" {
        println!("It's a banana");
    } else if fruit == "mango" {
        println!("It's a mango");
    } else {
        println!("It's  some other fruit");
    }

    
}