const PI: f64 = 3.1415926;

fn main() {
    let mut x = 5; // 기본적으로 mutable를 지정해주지 않으면 불변성을 띈다
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("PI= {PI}");

    let n = 10;

    let n = n + 1;

    // 섀도잉은 변수를 mut로 표시하는 것과는 다릅니다.
    {
        let n = n * 5;
        println!("n2: {n}");
    }

    println!("n1: {n}");

    /**
     * 섀도잉은 spaces_str과 spaces_num같이 구분되는 변수명을 쓸
     * 필요가 없도록 여유를 줍니다.
     * 아래를 봅시다.
     */
    let spaces = " ";
    let spaces = spaces.len();

}

