fn main() {
    let large_number1:u32 = 0xffff_fffe;
    let large_number2:u32 = 3000000000;
    
    println!("LN1: {}", large_number1);
    println!("LN2: {}", large_number2);
    
    let result = (large_number1 as u64) * (large_number2 as u64);
    println!("Mul. Result: {:e}", result as f64);
}
