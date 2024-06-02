use std::fmt;

use super::scanner;
use super::scanner::{Scanner, Token};
use super::vertex::{Collection, GameTree, Sequence, Node, Property};

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    Eof,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Error::ParseError(s) => s,
            Error::Eof => "EOF",
        };
        write!(f, "{}", t)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<scanner::Error> for Error {
    fn from(err: scanner::Error) -> Error {
        Error::ParseError(err.to_string())
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    cur: usize,
}

impl Parser {
    pub fn new(data: &str) -> Result<Self> {
        let tokens = Scanner::new(data).scan()?;
        let p = Parser {
            tokens: tokens,
            cur: 0,
        };
        Ok(p)
    }

    pub fn peek(&mut self, n: usize) -> Token {
        if self.cur < self.tokens.len() - n {
            self.tokens[self.cur + n].clone()
        } else {
            Token::Eof
        }
    }

    pub fn read(&mut self) -> Token {
        let ret = self.cur;

        if ret >= self.tokens.len() {
            self.cur += 1;
            return Token::Eof;
        }

        self.cur += 1;
        self.tokens[ret].clone()
    }

    pub fn create_error(&mut self, msg: &str) -> Error {
        if self.tokens.len() == 0 {
            return Error::ParseError(format!("empty file"));
        }
        if self.cur >= self.tokens.len() {
            return Error::ParseError(format!("parse_error at {}: {}", self.tokens[self.tokens.len()-1].position(), msg));
        }
        Error::ParseError(format!("parse_error at {}: {}", self.tokens[self.cur].position(), msg))
    }

    pub fn unexpected(&mut self, msg: &str) -> Error {
        let t = self.read();
        self.create_error(&format!("unexpected {} {}", t, msg))
    }

    pub fn consume_whitespace(&mut self) {
        loop {
            match self.peek(0) {
                Token::Whitespace | Token::Newline(_) => self.read(),
                _ => break,
            };
        }
    }

    pub fn parse(&mut self) -> Result<Collection> {
        self.consume_whitespace();
        let mut gametrees = Vec::new();

        // apparently kgs is ok with sgf files with garbage at the beginning
        // so i guess we'll do that too why not
        loop {
            match self.peek(0) {
                Token::OpenParen(_) => break,
                Token::Eof => break,
                _ => self.read(),
            };
        }

        loop {
            match self.peek(0) {
                Token::OpenParen(_) => gametrees.push(self.parse_gametree()?),
                _ => break,
            }
        }
        if gametrees.len() == 0 {
            return Err(self.create_error("cannot have empty collection"));
        }
        Ok(Collection{gametrees})
    }

    pub fn parse_gametree(&mut self) -> Result<GameTree> {
        // gametrees start with "("
        self.read();
        self.consume_whitespace();
        let seq = self.parse_sequence()?;
        self.consume_whitespace();
        let mut trees = Vec::new();
        loop {
            match self.peek(0) {
                Token::OpenParen(_) => {
                    trees.push(Box::new(self.parse_gametree()?));
                    self.consume_whitespace();
                }
                Token::CloseParen(_) => {
                    self.read();
                    break;
                }
                _ => return Err(self.unexpected("in parse_gametree")),
            }
        }
        Ok(GameTree{sequence: seq, gametrees: trees})
    }

    pub fn parse_sequence(&mut self) -> Result<Sequence> {
        // sequences start with node
        // nodes start with ";"
        let mut nodes = Vec::new();
        loop {
            match self.peek(0) {
                Token::Semicolon(_) => {
                    nodes.push(self.parse_node()?);
                    self.consume_whitespace();
                }
                _ => break,
            }
        }
        if nodes.len() == 0 {
            return Err(self.create_error("cannot have empty node list"));
        }
        Ok(Sequence{nodes})
    }

    pub fn parse_node(&mut self) -> Result<Node> {
        // nodes start with ";"
        self.read();
        self.consume_whitespace();
        let mut props = Vec::new();
        loop {
            match self.peek(0) {
                Token::UcLetter(..) => {
                    props.push(self.parse_property()?);
                    self.consume_whitespace();
                }
                _ => break,
            }
        }
        Ok(Node{props})
    }

    pub fn parse_property(&mut self) -> Result<Property> {
        let ident = self.parse_propident()?;
        self.consume_whitespace();
        let mut values = Vec::new();
        loop {
            match self.peek(0) {
                Token::OpenSquare(_) => {
                    values.push(self.parse_propvalue()?);
                    self.consume_whitespace();
                }
                _ => break,
            }
        }
        if values.len() == 0 {
            return Err(self.create_error("cannot have empty property list"));
        }
        Ok(Property{ident, values})
    }

    pub fn parse_propident(&mut self) -> Result<String> {
        match self.read() {
            Token::UcLetter(_, s) => Ok(s),
            _ => Err(self.create_error("expected uppercase identifier")),
        }
    }

    pub fn parse_propvalue(&mut self) -> Result<String> {
        match self.peek(0) {
            Token::OpenSquare(_) => self.read(),
            _ => return Err(self.unexpected("needed '[' in parse_propvalue")),
        };
        let mut s = "".to_owned();
        loop {
            match self.peek(0) {
                Token::CloseSquare(_) => break,
                Token::Eof => return Err(self.unexpected("eof while waiting for ']'")),
                t => {
                    s.push_str(&format!("{}", t));
                    self.read();
                }
            }
        }

        match self.peek(0) {
            Token::CloseSquare(_) => self.read(),
            _ => return Err(self.unexpected("needed ']' in parse_propvalue")),
        };
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse1() {
        let text = "(;GM[1])";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    #[test]
    fn parse2() {
		let text = "(;GM[1]AW[ab][bc])";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    #[test]
    fn parse3() {
		let text = "(;GM[1];B[cc])";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    #[test]
    fn parse4() {
		let text = "(;ZZ[aoeu [1k\\]])";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }
    
    #[test]
    fn parse5() {
		let text = "(;GM[1](;B[aa];W[ab])(;B[ab];W[ac]))";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    #[test]
    fn parse6() {
        let text = "
(;GM[1]FF[4]CA[UTF-8]AP[CGoban:3]ST[2]
RU[Japanese]SZ[19]KM[0.00]
PW[White]PB[Black]
AW[na][oa][pa][qa][ra][sa][ka][la][ma][ja]
AB[nb][ob][pb][qb][rb][sb][kb][lb][mb][jb]
LB[pa:A][ob:2][pb:B][pc:C][pd:D]
[oa:1][oc:3][ne:9][oe:8][pe:7][qe:6][re:5][se:4]
[nf:15][of:14][pf:13][qf:11][rf:12][sf:10]
[ng:22][og:44][pg:100]
[ka:a][kb:b][kc:c][kd:d][ke:e][kf:f][kg:g]
MA[na][nb][nc]
CR[qa][qb][qc]
TR[sa][sb][sc]
SQ[ra][rb][rc]
)";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    #[test]
    fn parse7() {
        let text = "
(;GM[1]FF[4]CA[UTF-8]AP[Glift]ST[2]
RU[Japanese]SZ[19]KM[0.00]
C[Black to play. There aren't many options
to choose from, but you might be surprised at the answer!]
PW[White]PB[Black]AW[pa][qa][nb][ob][qb][oc][pc][md][pd][ne][oe]
AB[na][ra][mb][rb][lc][qc][ld][od][qd][le][pe][qe][mf][nf][of][pg]
(;B[mc]
	;W[nc]C[White lives.])
(;B[ma]
	(;W[oa]
		;B[nc]
		;W[nd]
		;B[mc]C[White dies.]GB[1])
	(;W[mc]
		(;B[oa]
		;W[nd]
		;B[pb]C[White lives])
		(;B[nd]
			;W[nc]
			;B[oa]C[White dies.]GB[1]))
	(;W[nd]
		;B[mc]
		;W[oa]
		;B[nc]C[White dies.]GB[1]))
(;B[nc]
	;W[mc]C[White lives])
(;B[]C[A default consideration]
	;W[mc]C[White lives easily]))";
        let _ = Parser::new(text).unwrap().parse().unwrap();
    }

    /* error cases

        "(;GM[1](;B[aa]W[ab])(;B[ab];W[ac]))",
        "I am a banana",
        "(;C",
        "(;C)",
        "(;C[])())",
        "(;C[)())",
        "(;weird[])",
    */

    #[test]
    fn parse8() {
        let text = "";
        if let Ok(_) = Parser::new(text).unwrap().parse() {
            panic!();
        }
    }

    #[test]
    fn parse9() {
        let text = "\n";
        if let Ok(_) = Parser::new(text).unwrap().parse() {
            panic!();
        }
    }

    #[test]
    fn parse10() {
        let text = "\x28\x0a\x3b";
        if let Ok(_) = Parser::new(text).unwrap().parse() {
            panic!();
        }
    }

    #[test]
    fn parse11() {
        let text = "(;A[";
        if let Ok(_) = Parser::new(text).unwrap().parse(){
            panic!();
        }
    }

    #[test]
    fn parse12() {
        let text = "(;gm[1])";
        if let Ok(_) = Parser::new(text).unwrap().parse() {
            panic!();
        }
    }

    #[test]
    fn parse13() {
        let text = "(;[1])";
        if let Ok(_) = Parser::new(text).unwrap().parse() {
            panic!();
        }
    }
}
