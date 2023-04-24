use std::env;
use std::fs;


struct Config {
    query:String,
    file_name:String,
}
fn main() {
    let args:Vec<String>= env::args().collect();
    // println!("{:?}", args[0..]);
    // for i in args{
    //     println!("{:?}", i);
    // }

    // let query = &args[1];
    // let file_name = &args[2];
    
    
    let conf = parse_config(&args);
    let file_content = fs::read_to_string(&conf.file_name).expect("Failed to read the file");
    
    println!("Reading query {}", conf.query);
    println!("From file {}", conf.file_name);
    println!("File Content:- \n {:?}", file_content)
   
}

fn parse_config(args:&[String])-> Config {
    
    if args.len() < 3 {
        panic!("Enter proper details!")
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Config {
        query,
        file_name,
    }
}
