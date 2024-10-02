pub fn hexadecimal(){
    let decimal_num = 6789;
    let output1 = format!("decimal number {} in hex is {:#X}", decimal_num, decimal_num);
    let output2 = format!("decimal number {} in hex is {:#x}", decimal_num, decimal_num);
    let output3 = format!("decimal number {} in hex is {:x}", decimal_num, decimal_num);

    println!("{}\n{}\n{}", output1, output2, output3);
}