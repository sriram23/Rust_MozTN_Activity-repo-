use std::io;
fn main() {
    println!("Enter two numbers:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1)
            .expect("failed to read line");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2)
            .expect("failed to read line");
    
    let num1: u32 = num1.trim().parse() 
    .ok()
	.expect("Enter a valid number:");

    let num2: u32 = num2.trim().parse() 
    .ok()
	.expect("Enter a valid number:");
    println!("{}{}",num1,num2);

    println!("SUM: {}",num1 + num2);
    println!("DIFF: {}",num1 - num2);
    println!("MUL: {}",num1 * num2);
    println!("DIV: {}",num1 / num2);
}
