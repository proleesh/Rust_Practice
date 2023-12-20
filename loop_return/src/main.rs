fn main() {
    let mut counter = 0;

    let result = loop{
        counter += 1;
        
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0{
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("The value is: {}", a[index]);
        index += 1;
    }


        let a = [10, 20, 30, 40, 50];

        let var_name = for element in a {
            println!("the value is: {}", element);
        };
        var_name;

        for number in (1..4).rev(){
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
}
