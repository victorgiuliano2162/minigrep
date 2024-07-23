use std::{env, fs::File, io::prelude::*};

fn main() {

    let args: Vec<String> = env::args().collect();
    //if we need to use non unicode characters its necessary to use args_os instead that uses 
    //OsStrings rather than Strings. Otherwise OsStrings values differ to plattaform
    
    let query = &args[1];
    let filename = &args[2];
    
    println!("Searching for {}", query);
    
    println!("In file {}", filename);

    let mut f = File::open("").expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);

}
