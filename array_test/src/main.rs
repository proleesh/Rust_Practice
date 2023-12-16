fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", a[1]);

    another_function();
    a_number(5);


    print_labeled_measurement(23, 'm');


}
// rust에서 함수명은 일반 snake case를 사용한다.
// rust에서는 함수가 main함수 앞이냐 뒤에냐 관심없다.
// 아무데나 함수를 정의하고 쓰면된다.
fn another_function(){
    println!("Another Function");
}

// 아래처럼 의미있는 값을 전달하는 함수를 만들때 반드시 타입을 선언해야한다.
fn a_number(x: i32){
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {} {}", value, unit_label);
}