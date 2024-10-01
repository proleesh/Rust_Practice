use std::io;

pub fn calc_emi(){
    let mut input = String::new();
    let currency_symbol = '₩';

    println!("Enter the loan principal amount: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let principal: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    println!("Enter the annual interest rate (in percentage)");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let annual_rate: f64 = input.trim().parse().expect("Input number only!");
    input.clear();

    println!("Enter the loan term (int months):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let months: u32 = input.trim().parse().expect("Input number only!");
    input.clear();

    let emi = calculate_emi(principal, annual_rate, months);
    println!("Your EMI is: {}{:.2} for {} months", currency_symbol, emi, months);
}

fn calculate_emi(principal: f64, annual_rate: f64, months: u32) -> f64 {
    let monthly_rate = annual_rate / 12.0 / 100.0;
    let numerator = principal * monthly_rate * (1.0 + monthly_rate).powf(months as f64);
    let denominator = (1.0 + monthly_rate).powf(months as f64) - 1.0;
    numerator / denominator
}