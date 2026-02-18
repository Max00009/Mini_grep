use std::env;
use std::fs;

struct Config{
    query:String,
    filepath:String,
}
impl Config{
    fn new(args:&[String])->Config{
        let query=args[1].clone();
        let filepath=args[2].clone();
        Config{query,filepath}
    }
}
fn main(){
    let args:Vec<String>=env::args().collect();
    let config=Config::new(&args);
    println!("searching {} in {}",config.query,config.filepath);
    let contents=fs::read_to_string(config.filepath).expect("should have been opened the file");
}