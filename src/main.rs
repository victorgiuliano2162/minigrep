use std::{env, fs::{self, File}, io::prelude::*};

fn main() {

    /* let args: Vec<String> = env::args().collect();
    //if we need to use non unicode characters its necessary to use args_os instead that uses 
    //OsStrings rather than Strings. Otherwise OsStrings values differ to plattaform
    
    let query = &args[1];
    let filename = &args[2];
    
    println!("Searching for {}", query);
    
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents); */

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n{}", contents);

}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query =args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}