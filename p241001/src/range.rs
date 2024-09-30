pub fn range(){
    print!("(1..5): ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    print!("1..=5.rev: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();
}