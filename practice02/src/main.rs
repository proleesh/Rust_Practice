fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin,33
    Yellow-eyed penguin,65.8
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for(i, record) in records.enumerate(){
        // 첫 라인과 공백이면 뛰어넘기기
        if i == 0 || record.trim().len() == 0{
            continue;
        }

        // 필드 마다 공백을 제거한다
        let fields: Vec<_> = record
        .split(',')
        .map(|field|field.trim())
        .collect();
    // 구성(configuration)의 컴파일 시간을 체크
    if cfg!(debug_assertions){
        // eprintln!: prints to 표준 에러
        eprintln!("debug: {:?} -> {:?}",
    record, fields);
    }
    let name = fields[0];

    // f32 는 32 bit 부동 소수점을 의미
    // 이 코드에서 fields[1]는 즉 숫자 출력하는 위치를 지칭
    
    if let Ok(length) = fields[1].parse::<f32>(){
        println!("{}, {}cm", name, length);
    }

    }

}
