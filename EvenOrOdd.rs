fn main(){
    let mut input_buffer = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut input_buffer).expect("Error");
    let mut x:i32 = input_buffer.trim().parse().unwrap();
    if (x % 2 == 0){
        println!("{x} is Even");
    } else {
        println!("{x} is odd");
    }
}