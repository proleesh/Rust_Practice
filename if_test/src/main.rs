fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    if number == 3{
        println!("Number is {}", number);
    }

    let condition = true;
    let number2 = if condition {5} else {6};
    println!("The value of number is: {}", number2);


    loop{
        println!("again!");
    }
}
