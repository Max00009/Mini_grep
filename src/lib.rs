use std::error::Error;  //to handle error.
use std::fs;    //need to read file.
use std::env; //need to read environment variable

pub struct Config{
    pub query:String,
    pub filepath:String,
    pub ignore_case:bool,
}
impl Config{
    //associated function to parse the arguments.
    //insetad of creating a separate function we create a associate function cause this function is related to the config struct.It's for good abstract(as mentioned in Rust book)
    pub fn new(args:&[String])->Result<Config,&'static str>{    //static means the string's lifetime is as long as the program executes.
        if args.len()<3{
            return Err("not enough arguments"); //we are returning failure product=Err() here
        }
        let query=args[1].clone();
        let filepath=args[2].clone();
        let ignore_case=env::var("IGNORE_CASE").is_ok(); //env::var() returns a Result<Ok,Err>.we don't care about the error.we only want to know if it's ok.that's why we are using is_ok()
        Ok(Config{query,filepath,ignore_case}) //We are returning success product=Ok() here
    }
}
//This function is for the searching logic.
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{ //here we define liferime 'a and use that lifetime with contents argument and return type.
    //we need the return type to live as long as the contents live.cause our function will return reference to matched portion of contents.
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
    //first we need to lower the query
    let query=query.to_lowercase();
    let mut results=Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){ //here we lower the lines from contents.notice one thing we are passing '&' infront of query inside contains()
        //cause to_lowercase() returns a String but contains() expect a str slice.so we add '&' which is technically &String but Rust's Deref Coercion automatically converts the &String into a &str behind the scenes.
           results.push(line);
        }
    }

    results
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{   //Error is a trait.it means some type that behaves like an error.
    //dyn cause at compiletime we don't know exact what type it is.Box is for heap allocation cause trait objects have unknown size at compile time.
    let contents=fs::read_to_string(config.filepath)?;  //the ? operator converts errors automatically into Box<dyn Error>.
    //i.e. if fs::read_to_string() returns std::io::Error then it's automatically converted into Err(Box<dyn Error>).
    
    //we call function based on ignore_case bool value
    let results=if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }else{
        search(&config.query,&contents)       
    };
    
    for line in results{
        println!("{line}");
    }
    Ok(())
}

//now we will add tests cause it helps to check our functions are working before integrating everything.
#[cfg(test)]    //this is configuration attribute.it means compile this code only when running tests.i.e. cargo test.
mod tests{  //private test-only modules.
    use super::*;   //import everything from parent module.

    #[test] //case sensitive test function marker.
    fn case_sensitive(){
        let query="duck";
        let contents="\
duck off.
Duck off.
duCK off";
        assert_eq!(vec!["duck off."],search(query,contents));
    }

    #[test] //case insensitive test fucntion marker
    fn case_insensitive(){
        let query="duck";
        let contents="\
duck off.
Duck off.
duCK off.";
        assert_eq!(vec!["duck off.","Duck off.","duCK off."],search_case_insensitive(query,contents));
    }

}