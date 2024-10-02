fn calc_circle_area(_radius: f32) -> f32 {
    3.14 * 4.0 * 4.0
}
pub fn pi(){
    // let pi = 3.14;
    let pi = std::f64::consts::PI;

    let area = pi * 4.0 * 4.0;
    print!("area: {} ", area);
    println!();
    print!("area: {} ", calc_circle_area(10.0));
}