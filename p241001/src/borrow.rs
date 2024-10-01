pub fn borrow() {
    let mut x = 500;
    println!("The value of x is {}", x);
    let y = &mut x;
    *y = 100;
    println!("{}", x);
}
