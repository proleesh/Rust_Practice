fn main() {
    let s = "1. Hello";

    {
        let s = "2. Hello";
        println!("{}", s);
    }

    println!("{}", s);

    let mut s2 = String::from("3. Hello");
    s2.push_str(", world!");
    println!("{}", s2);

    let s_1 = String::from("Good");
    let s_2 = s_1.clone();
    println!("s_1 = {}, s_2 = {}", s_1, s_2);

    let x = 6;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
