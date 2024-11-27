fn main(){
    // Variables
    let guessing_number:u8 = 10;
    let mut hit_point:u8 = 3;

    // Main Event loop
    loop {
        // Input buffer
        let mut inputBuff = String::new();

        if (hit_point == 0){
            println!("Game over!");
            println!("Better luck next time");
            break;
        }
        println!("HP: {hit_point}");
        println!("What is the number: ");
        std::io::stdin().read_line(&mut inputBuff).expect("error");
        let mut number:u8 = inputBuff.trim().parse().unwrap();
        if (number == guessing_number){
            println!("Correct!!!!");
            println!("You Win!!!!!!!!!");
            break;
        } else {
            println!("\tTry Again");
            println!("--------------------------");
            hit_point -= 1;
        }
    }
}