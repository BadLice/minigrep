use std::env::{args};


fn main() {
    let args: Vec<String> = args().collect();

        if args.len() != 2 {
    eprintln!("Error: needed 2 args");
    }

    let path = &args[1];
    let query = &args[2];
    println!("path: {:?}", path);
    println!("path: {:?}", query);
}
