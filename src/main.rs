use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} [filename]", args[0]);
        return;
    }

    //let text = fs::read_to_string("examples/5.sgf").unwrap();
    let text = fs::read_to_string(&args[1]).unwrap();
    //let tokens = ast::scanner::Scanner::new(&text).scan().unwrap();
    //for tok in tokens {
    //    println!("{:?}", tok);
    //}
    let coll = ast::parser::Parser::new(&text).unwrap().parse().unwrap();
    println!("{}", coll);
}
