fn main() {
    println!("{}",std::env::consts::OS);
    println!("{}", std::env::consts::ARCH);
    println!("{}", std::env::consts::FAMILY);
    println!("{}", std::env::consts::EXE_EXTENSION);
    println!("{}", std::env::consts::EXE_SUFFIX);
}