fn main() {
    let guess: i32 = "42".parse().expect("Not a number!");

    println!("{}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("{}", x);
    println!("{}", y);

    // 덧셈
    let sum = 5 + 10;
    println!("{}",sum);

    // 뺄셈
    let difference: f64 = 43.2 - 2.4;
    println!("{}",difference);

    // 곱셈
    let product = 4 * 30;
    println!("{}",product);

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("{}",quotient);
    println!("{}",truncated);
    
    // 나머지 연산
    let remainder = 43 % 5;
    println!("{}", remainder);

    let t = true;

    // 명시적인 타입
    let f: bool = false;

    println!("{}",t);
    println!("{}",f);

    let c = 'z';
    let z: char = 'Z';
    let cat = '😸';

    print!("{} ", c);
    print!("{} ", z);
    print!("{}", cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of 500 is: {five_hundred}");
    println!("The value of 6.4 is: {six_point_four}");
    println!("The value of 1 is: {one}");


}
