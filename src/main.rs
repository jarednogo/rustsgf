use std::fs;
use std::env;

use sgf::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} [filename]", args[0]);
        return;
    }

    //let text = fs::read_to_string("examples/5.sgf").unwrap();
    let text = fs::read_to_string(&args[1]).unwrap();
    //let tokens = scanner::Scanner::new(&text).scan().unwrap();
    //for tok in tokens {
    //    println!("{:?}", tok);
    //}
    let coll = Parser::new(&text).unwrap().parse().unwrap();
    for gt in coll.gametrees {
        let gt2 = gt.strip_key("PB")
            .strip_key("PW")
            .strip_key("BR")
            .strip_key("WR");
        println!("{}", gt2);
    }
    //println!("{}", coll);
}
