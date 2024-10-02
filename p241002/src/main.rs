mod pi;
mod decimal;
mod hexadecimal;
mod binarydecimal;

fn main() {
    let array = [1,2,3];
/*    for i in 0..=2 {
        print!("{} ", array[i]);
    }*/
    for item in & array {
        print!("{} ", item);
    }

    println!();
    pi::pi();
    decimal::decimal();
    hexadecimal::hexadecimal();
    binarydecimal::binary_decimal();
}
