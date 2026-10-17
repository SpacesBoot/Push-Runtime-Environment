pub mod PRE;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filepath = &args[1];
    let code = std::fs::read_to_string(filepath).expect("file not found");

    println!("{}", crate::PRE::PRE::ExecuteBytecode(&code));
}