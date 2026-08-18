use std::env;   //to collect arguments.
use std::process;   //to terminate process with exit code.
use minigrep::{Config,run};
fn main(){
    let args:Vec<String>=env::args().collect(); //we collect the arguments from command line.
    let config=Config::new(&args).unwrap_or_else(|err|{  //we send the arguments to new function for parsing.
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    if let Err(e)=run(config){  //if run() returns Err intead of Config.
        eprintln!("Application error:{}",e);
        process::exit(1);
    };
}