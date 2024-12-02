fn sumOfN(upperLimit: u64) -> u64 {
    let mut sum:u64 = 0;
    for n in 1..=upperLimit {
        sum += n;
    }
    return sum;
}

fn main(){
    let mut inputBuff = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut inputBuff);
    let number: u64 = inputBuff.trim().parse().unwrap();
    println!("Sum of {} is {}", number, sumOfN(number));
}