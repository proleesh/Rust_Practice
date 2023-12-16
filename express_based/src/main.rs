fn main() {
    let f = six();
    let a = one();
    let y = {
        let x = 999;
        x + a + f
    };
    println!("The value of y is: {}", y);

    let p_one = plus_one(9);
    println!("The value of p_one is: {}", p_one);
}

fn six() -> i32{
    6
}

fn one() -> i32{
    1
}

fn plus_one(x: i32) -> i32{
    x + 1
}
