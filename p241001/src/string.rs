pub fn string(){
    let mut s = String::from("Hello");
    let t = String::from("World");
    s = t;
    println!("s is {}",s);
    // println!("t is {}",t);
}