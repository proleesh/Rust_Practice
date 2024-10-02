pub fn r_crosshatch_tag(){
    let code = r####"
    let num1 = 100;
    let num2 = 200;
    let sum = num1 + num2;
    println!("{} + {} = {}", num1, num2, sum);
    "####;
    println!("{}", code);
}