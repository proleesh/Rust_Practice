use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main(){
    let num1 = 10;
    let num2 = Box::new(20);
    let num3 = Rc::new(Box::new(30));
    let num4 = Arc::new(Mutex::new(40));

    println!("num1: {:?}, num2: {:?}, num3: {:?}, num4: {:?}",
num1,num2,num3,num4);

}