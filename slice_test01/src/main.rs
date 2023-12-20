fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);
    s.clear();

    let s2 = String::from("great");
    let len = s2.len();

    let slice1 = &s2[0..5];
    let slice2 = &s2[..len];
    let slice3 = &s2[..5];

    println!("{}", slice1);
    println!("{}", slice2);
    println!("{}", slice3);
}

fn first_word(s: &String) -> usize{
    // 문자열값이 공백인지 확인하기 위해 문자열을 byte로 전환
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}
