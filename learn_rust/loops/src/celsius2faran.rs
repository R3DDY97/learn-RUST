
use std::io;

fn main() {
    cel2faren();
}


fn cel2faren() {
    
    println!("enter celsius as float  ");
        
        let mut cel = String::new();
    io::stdin().read_line(&mut cel)
        .expect("failed to read line");

    let cel: f32 = cel.trim().parse()
        .expect("please enter float");
       
    let faren = (cel * 1.8) + 32.00;
    
    println!("The farenheit value of {} celsius is {}", cel, faren);
}


