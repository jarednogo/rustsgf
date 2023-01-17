use std::fs;

fn main() {
    let text = fs::read_to_string("examples/5.sgf").unwrap();
    //let tokens = ast::scanner::Scanner::new(&text).scan().unwrap();
    //for tok in tokens {
    //    println!("{:?}", tok);
    //}
    let coll = ast::parser::Parser::new(&text).unwrap().parse().unwrap();
    println!("{}", coll);
}
