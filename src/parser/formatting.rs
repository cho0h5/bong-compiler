// 이 파일에서는 여러 자료형의 출력을 가독성 좋게 하기 위해
// dummy struct(Tokens, Tree)를 정의하고
// 그 자료형의 출력을 구현하고있습니다.

use std::fmt;
use std::collections::VecDeque;

use crate::parser::Node;
use crate::parser::Node::*;

// token(Node)들의 배열을 나타내는 dummy struct인 Token을 정의합니다.
pub struct Tokens(pub VecDeque<Node>);

// Tokens의 출력을 정의합니다.
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").ok();
        if self.0.len() != 0 {
            let mut iter = self.0.iter();
            let token = match iter.next().unwrap() {
                Terminal(token) => token,
                NonTerminal(token, _) => token,
            };
            write!(f, "{:?}", token).ok();

            for node in iter {
                let token = match node {
                    Terminal(token) => token,
                    NonTerminal(token, _) => token,
                };
                write!(f, " {:?}", token).ok();
            }
        }
        write!(f, "]").ok();
        Ok(())
    }
}

// Node를 wrapping하는 Tree를 정의합니다.
pub struct Tree(pub Node);

// Tree의 출력을 정의합니다.
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f, true, vec![]);
        Ok(())
    }
}

// Node의 출력을 정의합니다.
// fmt(...)함수가 재귀적으로 호출되며 tree를 출력합니다.
impl Node {
    fn fmt(&self, f: &mut fmt::Formatter, is_last: bool, bridge: Vec<bool>) {
        match self {
            Terminal(token) => {
                for b in &bridge[..bridge.len()-1] {
                    write!(f, "{}", if *b { "│   " } else { "    " }).ok();
                }
                write!(f, "{}", if is_last { "└── " } else { "├── " }).ok();
                write!(f, "{:?}\n", token).ok();
            },
            NonTerminal(token, children) => {
                if bridge.len() != 0 {
                    for b in &bridge[..bridge.len()-1] {
                        write!(f, "{}", if *b { "│   " } else { "    " }).ok();
                    }
                    write!(f, "{}", if is_last { "└── " } else { "├── " }).ok();
                }
                write!(f, "\x1b[36m{:?}\x1b[97m\n", token).ok();
                for i in 0..children.len() {
                    let mut _bridge = bridge.clone();
                    _bridge.push(i != children.len() - 1);
                    children[i].fmt(f, i == children.len() - 1, _bridge);
                }
            }
        };
    }
}
