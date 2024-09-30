fn main() {
    let x:i32 = 3;
    let x = x + 2;
    let x = x * 2;
    println!("x: {}", x);

    let x = "Hello, Rust!";
    println!("x: {}", x);

    let integer1: u32 = 17;
    let integer2 = 17u32;
    let integer3 = 17;
    let integer4: u32 = 0b10001;
    let integer5: u32 = 0o21;
    let integer6: u32 = 0x11;
    let integer7 = 50_000;
    println!("integer1: {}", integer1);
    println!("integer2: {}", integer2);
    println!("integer3: {}", integer3);
    println!("integer4: {}", integer4);
    println!("integer5: {}", integer5);
    println!("integer6: {}", integer6);
    println!("integer7: {}", integer7);
}
