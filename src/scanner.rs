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
    UcLetter(Position, String),

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
    Bytes(Position, String),

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
            Token::UcLetter(pos, _) => *pos,
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
            Token::Bytes(pos, _) => *pos,
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
            Token::UcLetter(_, s) => write!(f, "{}", s),
            Token::OpenParen(_) => write!(f, "("),
            Token::CloseParen(_) => write!(f, ")"),
            Token::OpenSquare(_) => write!(f, "["),
            Token::CloseSquare(_) => write!(f, "]"),
            Token::Semicolon(_) => write!(f, ";"),
            Token::Float(_, d) => write!(f, "{}", d),
            Token::Integer(_, i) => write!(f, "{}", i),
            Token::Escaped(_, s) => write!(f, "\\{}", s),
            Token::Ascii(_, s) => write!(f, "{}", s),
            Token::Bytes(_, s) => write!(f, "{}", s),
        }
    }
}

pub struct Scanner {
    input: Vec<char>,
    cur: usize,
    pos: Position,
}

impl Scanner {
    pub fn new(data: &str) -> Self {
        Scanner {
            //input: data.iter().map(|b| *b as char).collect::<Vec<_>>(),
            input: data.chars().collect(),
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
            _ => self.scan_bytes(),
            //c => Err(self.create_error(format!("invalid character: {}", c))),
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

    pub fn scan_bytes(&mut self) -> Result<Token> {
        let mut char_vec: Vec<char> = Vec::new();
        char_vec.push(self.read());
        let s: String = char_vec.into_iter().collect();
        Ok(Token::Bytes(self.pos, s))
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
        let mut upper = true;
        let mut char_vec: Vec<char> = Vec::new();
        loop {
            let c = self.peek(0);
            if is_identifier(c) {
                if c < 'A' || c > 'Z' {
                    upper = false;
                }

                char_vec.push(c);
                self.read();
            } else {
                break;
            }
        }
        let s: String = char_vec.into_iter().collect();
        match s.as_str() {
            _ => {
                if upper {
                    return Ok(Token::UcLetter(self.pos, s));
                } else {
                    return Ok(Token::Identifier(self.pos, s));
                }
            },
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
        let _ = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan2() {
		let text = "(;GM[1]AW[ab][bc])";
        let _ = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan3() {
		let text = "(;GM[1];B[cc])";
        let _ = Scanner::new(text).scan().unwrap();
    }

    #[test]
    fn scan4() {
		let text = "(;ZZ[aoeu [1k\\]])";
        let _ = Scanner::new(text).scan().unwrap();
    }
    
    #[test]
    fn scan5() {
		let text = "(;GM[1](;B[aa];W[ab])(;B[ab];W[ac]))";
        let _ = Scanner::new(text).scan().unwrap();
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
        let _ = Scanner::new(text).scan().unwrap();
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
        let _ = Scanner::new(text).scan().unwrap();
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
    fn scan8() {
        let text = "
(;GM[1]FF[4]
SZ[19]
GN[]
DT[2024-04-11]
PB[老朽006]
PW[sokka]
BR[5段]
WR[5段]
KM[375]HA[0]RU[Chinese]AP[GNU Go:3.8]RN[3]RE[B+R]TM[1200]TC[3]TT[60]AP[foxwq]RL[0]
;B[pd];W[dd];B[pq];W[dq];B[fc];W[hc];B[cc];W[dc];B[cd];W[de];B[db];W[eb];B[cb];W[fb];B[cf];W[nc];B[qf];W[ne];B[do];W[co];B[cn];W[cp];B[dn];W[fq];B[dj];W[qo];B[op];W[eg];B[ch];W[df];B[cg];W[pg];B[qg];W[pi];B[ob];W[nb];B[pn];W[qm];B[pm];W[ql];B[jg];W[je];B[ri];W[ji];B[ih];W[ej];B[ii];W[dk];B[ek];W[cj];B[di];W[el];B[fk];W[cl];B[fm];W[em];B[fn];W[en];B[eo];W[fl];B[gl];W[gk];B[fj];W[gm];B[hl];W[fo];B[gn];W[bl];B[rp];W[ro];B[qp];W[jq];B[qj];W[pl];B[ok];W[ol];B[nn];W[nk];B[oj];W[nm];B[nj];W[mn];B[no];W[mk];B[lm];W[mm];B[ll];W[mj];B[mi];W[ln];B[kp];W[li];B[mh];W[km];B[kl];W[jl];B[jm];W[kn];B[jk];W[jn];B[hm];W[go];B[ho];W[jp];B[hq];W[im];B[ep];W[gq];B[eq];W[er];B[fr];W[gr];B[dr];W[fs];B[cq];W[bq];B[dp];W[bn];B[cm];W[dl];B[bm];W[bo];B[am];W[cr];B[br];W[dq];B[bj];W[al];B[cq];W[fi];B[ei];W[dq];B[gc];W[gb];B[cq];W[gj];B[ej];W[dq];B[hd];W[ic];B[cq];W[in];B[hn];W[dq];B[ge];W[ec];B[cq];W[ci];B[bi];W[dq];B[ck];W[fp];B[cq];W[cs];B[ak];W[dq];B[bk];W[hf];B[gg];W[gf];B[fg];W[ff];B[hg];W[lf];B[kq];W[lh];B[qk];W[mg];B[pb];W[kk];B[jj];W[ni];B[na];W[ma];B[oa];W[lb];B[if];W[ie];B[il];W[jm];B[kj];W[lk];B[jr];W[ir];B[kr])";
        let _ = Scanner::new(text).scan().unwrap();
    }

}
