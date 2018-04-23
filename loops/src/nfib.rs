
use std::io;

fn main(){

    println!("Enter number to get nth fibonacci number");
    
    let mut n = String::new();
    
    io::stdin().read_line(&mut n)
        .expect("Not able to readline");

    let n = n.trim().parse()
        .expect("Please enter postive number");

    let mut a: u64 = 1;
    let mut b: u64 = 1;

    let mut count = 1;

    while count < n {
         let temp = a;
         a = b;
        b = temp.wrapping_add(a);
        count += 1;
        println!("{}", b);
                
    }

    println!("The nth fib {}", b);    

}
