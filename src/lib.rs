use std::error::Error;  //to handle error.
use std::fs;    //need to read file.

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
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{ //here we define liferime 'a and use that lifetime with contents argument and return type.
    //we need the return type to live as long as the contents live.cause our function will return reference to matched portion of contents.
    let mut result=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //Error is a trait.it means some type that behaves like an error.
    //dyn cause at compiletime we don't know exact what type it is.Box is for heap allocation cause trait objects have unknown size at compile time.
    let contents=fs::read_to_string(config.filepath)?;  //the ? operator converts errors automatically into Box<dyn Error>.
    //i.e. if fs::read_to_string() returns std::io::Error then it's automatically converted into Err(Box<dyn Error>).
    for line in search(&config.query,&contents){
        println!("{line}");
    }
    Ok(())
}

//now we will add tests cause it helps to check our functions are working before integrating everything.
#[cfg(test)]    //this is configuration attribute.it means compile this code only when running tests.i.e. cargo test.
mod tests{  //private test-only modules.
    use super::*;   //import everything from parent module.

    #[test] //test function marker.
    fn one_result(){
        let query="duck";
        let contents="duck off";
        assert_eq!(vec!["duck off"],search(query,contents));
    }
}