fn justSomeTest(testNumber: i32) -> i32 {
    return testNumber;
}

fn justAnotherTest(testNumber: i32) -> (){
    println!("Just some number again: {testNumber}");
}

fn main() {
    println!("{} is a test", justSomeTest(32));
    justAnotherTest(32);
}