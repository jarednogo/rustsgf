use std::fmt;

#[derive(Debug)]
pub enum Error {
    ScanError(String),
    Eof,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Error::ScanError(s) => s,
            Error::Eof => "EOF",
        };
        write!(f, "{}", t)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ScanError(err.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: u32,
    pub col: u32,
}

impl PartialEq for Position {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.row, self.col)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eof,
    Whitespace,
    Newline(Position),

    Identifier(Position, String),

    // symbols
    OpenParen(Position),
    CloseParen(Position),
    OpenSquare(Position),
    CloseSquare(Position),
    Semicolon(Position),

    // number types
    Float(Position, f64),
    Integer(Position, u64),

    Escaped(Position, String),

    Ascii(Position, String),

    /*
    UcLetter(Position, String),
    Digit(Position, u64),
    Real(Position, f64),
    */
}

impl Token {
    pub fn position(&self) -> Position {
        match self {
            Token::Eof => Position {row: 0, col: 0},
            Token::Whitespace => Position {row: 0, col: 0},
            Token::Identifier(pos, _) => *pos,
            Token::Newline(pos) => *pos,
            Token::OpenParen(pos) => *pos,
            Token::CloseParen(pos) => *pos,
            Token::OpenSquare(pos) => *pos,
            Token::CloseSquare(pos) => *pos,
            Token::Semicolon(pos) => *pos,
            Token::Float(pos, _) => *pos,
            Token::Integer(pos, _) => *pos,
            Token::Escaped(pos, _) => *pos,
            Token::Ascii(pos, _) => *pos,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eof => write!(f, ""),
            Token::Whitespace => write!(f, " "),
            Token::Newline(_) => write!(f, "\n"),
            Token::Identifier(_, s) => write!(f, "{}", s),
            Token::OpenParen(_) => write!(f, "("),
            Token::CloseParen(_) => write!(f, ")"),
            Token::OpenSquare(_) => write!(f, "["),
            Token::CloseSquare(_) => write!(f, "]"),
            Token::Semicolon(_) => write!(f, ";"),
            Token::Float(_, d) => write!(f, "{}", d),
            Token::Integer(_, i) => write!(f, "{}", i),
            Token::Escaped(_, s) => write!(f, "\\{}", s),
            Token::Ascii(_, s) => write!(f, "{}", s),
        }
    }
}

pub struct Scanner {
    input: Vec<char>,
    cur: usize,
    pos: Position,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        Scanner {
            input: s.chars().collect(),
            cur: 0,
            pos: Position {row: 1, col: 0},
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];
        loop {
            match self.scan_token() {
                Ok(Token::Eof) => break,
                //Ok(Token::Whitespace) | Ok(Token::Newline(_)) => continue,
                Ok(tok) => tokens.push(tok),
                Err(e) => return Err(self.create_error(e.to_string())),
            }
        }
        Ok(tokens)
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        // this should be comprehensive
        let token = match self.peek(0) {
            '\0' => Ok(Token::Eof),
            ' ' | '\t' | '\r' => self.scan_whitespace(),
            '\n' => self.scan_newlines(),
            '\\' => self.scan_escaped(),
            '(' => self.create_token(Token::OpenParen(self.pos)),
            ')' => self.create_token(Token::CloseParen(self.pos)),

            '[' => self.create_token(Token::OpenSquare(self.pos)),
            ']' => self.create_token(Token::CloseSquare(self.pos)),

            '0'..='9' => self.scan_number(),
            'a'..='z'|'A'..='Z'|'_' => self.scan_identifier(),
            ';' => self.create_token(Token::Semicolon(self.pos)),
            '\u{20}'..='\u{7e}' => self.scan_ascii(),
            c => Err(self.create_error(format!("invalid character: {}", c))),
        };
        token
    }

    pub fn create_error(&mut self, msg: String) -> Error {
        Error::ScanError(format!("scan_error at {}: {}", self.pos, msg))
    }

    pub fn create_token(&mut self, tok: Token) -> Result<Token> {
        self.read();
        Ok(tok)
    }

    pub fn peek(&mut self, n: usize) -> char {
        if self.cur < self.input.len() - n {
            self.input[self.cur + n]
        } else {
            '\0'
        }
    }

    pub fn read(&mut self) -> char {
        let ret = self.cur;

        if ret >= self.input.len() {
            self.cur += 1;
            return '\0';
        }

        if self.input[ret] == '\n' {
            self.pos.row += 1;
            self.pos.col = 0;
        } else {
            self.pos.col += 1;
        }

        self.cur += 1;
        self.input[ret]
    }

    pub fn scan_whitespace(&mut self) -> Result<Token> {
        loop {
            match self.peek(0) {
                ' ' | '\t' | '\r' => self.read(),
                _ => break,
            };
        }
        Ok(Token::Whitespace)
    }

    pub fn scan_newlines(&mut self) -> Result<Token> {
        loop {
            match self.peek(0) {
                '\n' => self.read(),
                _ => break,
            };
        };
        Ok(Token::Newline(self.pos))
    }

    pub fn scan_escaped(&mut self) -> Result<Token> {
        self.read();
        let mut char_vec: Vec<char> = Vec::new();
        char_vec.push(self.read());
        let s: String = char_vec.into_iter().collect();
        Ok(Token::Escaped(self.pos, s))
    }

    pub fn scan_ascii(&mut self) -> Result<Token> {
        let mut char_vec: Vec<char> = Vec::new();
        char_vec.push(self.read());
        let s: String = char_vec.into_iter().collect();
        Ok(Token::Ascii(self.pos, s))
    }

    pub fn scan_number(&mut self) -> Result<Token> {
        let mut char_vec: Vec<char> = Vec::new();
        loop {
            match self.peek(0) {
                '0'..='9' => char_vec.push(self.read()),
                _ => break,
            }
        }
        let s: String = char_vec.into_iter().collect();
        let n: u64 = s.parse()?;
        Ok(Token::Integer(self.pos, n))
    }

    pub fn scan_identifier(&mut self) -> Result<Token> {
        let mut char_vec: Vec<char> = Vec::new();
        loop {
            let c = self.peek(0);
            if is_identifier(c) {
                char_vec.push(c);
                self.read();
            } else {
                break;
            }
        }
        let s: String = char_vec.into_iter().collect();
        match s.as_str() {
            _ => Ok(Token::Identifier(self.pos, s)),
        }
    }
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9'
}

fn is_identifier_start(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_identifier(c: char) -> bool {
    return is_digit(c) || is_identifier_start(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan1() {
        let text = "(;GM[1])";
        let tokens = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan2() {
		let text = "(;GM[1]AW[ab][bc])";
        let tokens = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan3() {
		let text = "(;GM[1];B[cc])";
        let tokens = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan4() {
		let text = "(;ZZ[aoeu [1k\\]])";
        let tokens = Scanner::new(text).scan().unwrap();
    }
    
    #[test]
    fn scan5() {
		let text = "(;GM[1](;B[aa];W[ab])(;B[ab];W[ac]))";
        let tokens = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan6() {
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
        let tokens = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan7() {
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
        let tokens = Scanner::new(text).scan().unwrap();
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
}
