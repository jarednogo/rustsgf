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

impl GameTree {
    pub fn strip_key(&self, key: &str) -> Self {
        let mut gametrees = Vec::new();
        for gt in &self.gametrees {
            gametrees.push(Box::new(gt.strip_key(key)));
        }
        GameTree{
            sequence: self.sequence.strip_key(key),
            gametrees: gametrees,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub nodes: Vec<Node>,
}

impl Sequence {
    pub fn strip_key(&self, key: &str) -> Self {
        let mut nodes = Vec::new();
        for node in &self.nodes {
            nodes.push(node.strip_key(key));
        }
        Sequence{
            nodes: nodes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub props: Vec<Property>,
}

impl Node {
    pub fn strip_key(&self, key: &str) -> Self {
        let mut props = Vec::new();
        for prop in &self.props {
            props.push(prop.strip_key(key));
        }
        Node{
            props: props,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub ident: String,
    pub values: Vec<String>,
}

impl Property {
    pub fn strip_key(&self, key: &str) -> Self {
        let mut values = Vec::new();
        if self.ident.as_str() != key {
            for v in &self.values {
                values.push(v.clone().to_string());
            }
        } else {
            values.push("".to_string());
        }
        Property{
            ident: self.ident.clone(),
            values: values,
        }
    }
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
