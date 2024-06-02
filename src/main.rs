use std::fs;
use std::env;
use std::io::ErrorKind;

use sgf::parser::Parser;

fn filter_ascii(data: Vec<u8>) -> String {
    let mut d: Vec<u8> = Vec::new();
    for b in data {
        if b <= 0x7f {
            d.push(b);
        }
    }
    String::from_utf8(d.clone()).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} [filename]", args[0]);
        return;
    }

    //let text = fs::read_to_string("examples/5.sgf").unwrap();
    let data = match fs::read_to_string(&args[1]) {
        Ok(data) => data,
        Err(err) => match err.kind() {
            ErrorKind::InvalidData => filter_ascii(fs::read(&args[1]).unwrap()),
            e => panic!("{:?}", e),
        },
    };
        
    //let data = fs::read_to_string(&args[1]).unwrap();
    //let tokens = scanner::Scanner::new(&text).scan().unwrap();
    //for tok in tokens {
    //    println!("{:?}", tok);
    //}
    let coll = Parser::new(&data).unwrap().parse().unwrap();
    //for gt in coll.gametrees {
    //    let gt2 = gt.strip_key("PB")
    //        .strip_key("PW")
    //        .strip_key("BR")
    //        .strip_key("WR");
    //    println!("{}", gt2);
    //}
    println!("{}", coll);
}
