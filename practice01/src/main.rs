/*
Rust 언어에서는  String값의 범위가 넓고 비영어 인코딩을 지원하기 때문에
개발자 입장에서 안심하게 사용할 수 있다.
 */
fn greet_korean() {
    println!("Hello, world!");
    // Assignment 작업시 반드시 let 라는 키워드 사용
    let seongnam = "성남시";
    let korean = "안녕하세요!";
    let regions = [seongnam, korean];
    // iter() 메서드 사용시 반복문 가능
    for region in regions.iter(){
        // &ampersand 는 한번만 접근 가능하게 한다.
        println!("{}", &region);
    }
}


fn main(){
    greet_korean();
}