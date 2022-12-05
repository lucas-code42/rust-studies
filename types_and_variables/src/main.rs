fn main() {
    // : i8 int 8 bits
    // : i16 int 16 bits
    // : i32 int 32 bits
    // : i64 int 64 bits
    // : i128 int 128 bits
    // : u16 int 128 bits unsigned (apenas suporta valores positivos)
    
    let x: i32 = 32; // int 32 bits
    let f: f32 = 9.9; // float 32 bits
    let b: bool = false;

    println!("Valor int 32 = {}", x);
    println!("Valor float 32 = {}", f);
    println!("Valor booleano = {}", b);


    let _test: i32; // valor que não será ultilado inicia com _
    let mut test1: i32 = 0;
    let mut count: u32 = 0;

    loop {
        count += 1;
        if count == 3 {
            break;
        }
        test1 += 1;
    }
    
    println!("{}", test1);
}
