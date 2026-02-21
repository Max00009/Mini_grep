use std::error::Error;
use std::fs;

pub struct Config{
    query:String,
    filepath:String,
}
impl Config{
    //associated function to parse the arguments.
    //insetad of creating a separate function we create a associate function cause this function is related to the config struct.It's for good abstract(as mentioned in Rust book.idk lol).
    pub fn new(args:&[String])->Result<Config,&'static str>{    //static means the string's lifetime is as long as the program executes.
        if args.len()<3{
            return Err("not enough arguments");
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        Ok(Config{query,filepath})
    }
}
//This function is for the searching logic.
pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //Error is a trait.it means some type that behaves like an error.
    //dyn cause at compiletime we don't know exact what type it is.Box is for heap allocation cause trait objects have unknown size at compile time.
    println!("Searching {} in {}",config.query,config.filepath);
    let contents=fs::read_to_string(config.filepath)?;  //the ? operator converts errors automatically into Box<dyn Error>.
    //i.e. if fs::read_to_string() returns std::io::Error then it's automatically converted into Err(Box<dyn Error>).
    println!("with text:\n{contents}");
    Ok(())
}