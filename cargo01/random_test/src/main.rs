use std::io;
use rand::Rng;


fn main() {
    println!("숫자 예측!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("비밀의 번호는 {}", secret_number);

    println!("아무 숫자를 입력하시오> ");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("읽어들 일 수 없습니다.");


    println!("당신이 입력한 숫자는 {}", guess);
}
