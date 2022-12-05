// import lib's
use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");

    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("Number1 {number1} eh maior que {number2}", number1=number1, number2=number2)
    } else {
        println!("Number1 {} eh menor ou igual que {}", number1, number2)
    }
}
