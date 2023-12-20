fn main() {

    let s1 = gives_ownership();

    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    let n1 = 18;

    let n2 = return_a_number(n1);

    println!("{}", n2);
    
}

fn gives_ownership() -> String{

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn return_a_number(a_number: i32) -> i32{
    a_number
}