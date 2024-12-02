fn main(){
    let mut someVariable: i32 = 111;
    println!("Number Conversion");
    println!("Variable: {someVariable}");
    println!("Binary: {:b}",someVariable);
    println!("Octal: {:o}", someVariable);
    println!("Hex: {:x}", someVariable);
    println!("=================================");
    println!("Padded Output");
    println!("{:0>10}", 1);
    println!("{:0>10}", 2);
    println!("{:0>10}", 3);
}