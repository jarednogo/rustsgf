use std::fmt;

#[derive(Debug, Clone)]
pub struct Collection {
    pub gametrees: Vec<GameTree>,
}

#[derive(Debug, Clone)]
pub struct GameTree {
    pub sequence: Sequence,
    pub gametrees: Vec<Box<GameTree>>,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Node  {
    pub props: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub ident: String,
    pub values: Vec<String>,
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        for gt in &self.gametrees {
            s.push_str(&format!("{}", gt));
        }
        write!(f, "{}", s)
    }
}

impl fmt::Display for GameTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "(".to_owned();
        s.push_str(&format!("{}", self.sequence));
        for gt in &self.gametrees {
            s.push_str(&format!("{}", gt));
        }
        s.push_str(")");
        write!(f, "{}", s)
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        for node in &self.nodes {
            s.push_str(&format!("{}", node));
        }
        write!(f, "{}", s)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        for prop in &self.props {
            s.push_str(&format!("{}", prop));
        }
        write!(f, ";{}", s)
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        s.push_str(&self.ident);
        for value in &self.values {
            s.push_str(&format!("[{}]", value));
        }
        write!(f, "{}", s)
    }
}
