fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    println!("Hello, world!");
}
