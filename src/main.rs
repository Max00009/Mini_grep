use std::env;   //to collect arguments.
use std::process;   //to terminate process with exit code.
use minigrep::{Config,run};
fn main(){
    let config=Config::build(env::args()).unwrap_or_else(|err|{  //env::args() returns iterator which we pass to build function for parsing.here unwrap_or_else() takes a closure.
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    if let Err(e)=run(config){  //if run() returns Err intead of Config.
        eprintln!("Application error:{}",e);
        process::exit(1);
    };
}