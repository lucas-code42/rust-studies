fn main() {
    let number1 = 24;
    let number2 = 42;

    if number1 > number2 {
        println!("{} > {}", number1, number2);
    } else {
        println!("{} <= {}", number1, number2);
    }

    if true == false {
        print!("Equals")
    } else if false != true {
        print!("Dif")
    }
}
