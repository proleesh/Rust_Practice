use std::io;

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut: mutability(가변성) rust에서 기본은 불변성
    // 예: let guess는 불변 즉 수정불가.
    let mut guess = String::new();

    // std::io::stdin() 만약에 std::io를 선언 안했을시
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line"); // 반드시 추가

    println!("You guessed: {}", guess);
}