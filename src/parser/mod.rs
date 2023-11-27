// 이 파일에서는 제일 중요한 parse(...) 함수를 구현하였습니다.

// parsing_table.rs 파일에는 parsing에 필요한 CFG와 parsing_table이
// hard coding 되어 있습니다.
mod parsing_table;

// formatting.rs 파일에는 콘솔 출력을 가독성 좋게 하기 위한 자료형과 함수들이
// 구현되어있습니다.
pub mod formatting;

use std::collections::VecDeque;

use crate::token_reader::Token;

use crate::parser::parsing_table::TableElement::*;
use crate::parser::parsing_table::Reduction;

use crate::parser::formatting::Tokens;
use crate::parser::formatting::Tree;

use Node::*;

// parse(...) 함수가 parsing에 실패하였을 때 return 하는 error를 나타내는 struct입니다.
// pub Vec<Token>은 expected token을 나타내고
// pub Token은 parsing에 found된 token의 정보를 담습니다.
pub struct ParsingError(pub Vec<Token>, pub Token);

// parsing tree를 나타내기 위한 node를 표현하는 enum입니다.
// 모든 node는 Terminal이거나 NonTerminal이며
// NonTerminal의 경우 재귀적으로 Node들의 벡터를 가집니다.
#[derive(Debug)]
pub enum Node {
    Terminal(Token),
    NonTerminal(Token, Vec<Node>),
}

// 구현의 단순화를 위해 stack에 state뿐만 아니라 parsing중인 node들도 같이
// 담도록 하였습니다.
#[derive(Debug)]
struct StackItem {
    state: usize,
    tree: Option<Node>,
}

impl StackItem {
    // 새로운 StckItem을 생성하는 함수입니다.
    fn from(state: usize, tree: Option<Node>) -> Self {
        StackItem { state: state, tree: tree }
    }
}

// Tokens를 parsing하여 Tree또는 ParsingError를 return하는 함수입니다.
// parsing에 성공하면 return되는 Tree자료형은 src/parser/formatting.rs에
// 정의되어 있습니다. 그 형태는 단순하게 Node를 wrapping하고있으며
// 출력 formatting을 위해 정의되었습니다.
// parsing에 실패하면 ParsingError struct에 어떤 token들이 expected되는지,
// 어떤 token이 인식되었는지 정보를 담아 return합니다.
pub fn parse(tokens: Tokens) -> Result<Tree, ParsingError> {
    let mut tokens = tokens.0;

    // parsing_table::get_parsing_table()은 src/parser/parsing_table.rs에 정의되어 있습니다.
    // hard coding된 parsing table을 return하는 함수입니다.
    let parsing_table = parsing_table::get_parsing_table();

    // parsing_table::get_reduction_table()은 reduction을 할 때 필요한 CFG를  return하는
    // 함수이며 나머지는 parsing_table::get_parsing_table() 함수와 같습니다.
    let reduction_table = parsing_table::get_reduction_table();

    // stack을 생성한 후 state 0을 넣습니다.
    let mut stack = vec![StackItem::from(0, None)];

    loop {
        // parsing table을 검색하기 위해 현재 state와 next token 정보를 가져옵니다.
        let current_state = stack.last().unwrap().state;
        let next_token = match tokens[0] {
            Terminal(token) => token,
            NonTerminal(token, _) => token,
        };

        // parsing_table에 현재 state와 next token에 맞는 rule이 있다면
        // 해당 rule을 처리합니다.
        match parsing_table[current_state].get(&next_token) {
            Some(behavior) => match behavior{
                // 찾아진 rule에 따라 parsing을 진행합니다.
                // shift와 goto를 하나의 함수로 구현하여 구현을 단순화 하였습니다.
                // Shift, Reduce, Goto, Accepted 는 src/parser/parsing_table.rs에 TableElement로
                // 정의되어 있으며, parsing table의 rule을 나타냅니다.
                Shift(next_state) => shift_goto(&mut tokens, &mut stack, *next_state),
                Reduce(reduction_index) => reduce(&mut tokens, &mut stack, reduction_table[*reduction_index]),
                Goto(next_state) => shift_goto(&mut tokens, &mut stack, *next_state),
                Accepted => break,
            },
            None => {
                // 현재 state와 next token에 맞는 rule이 없다면 부가 정보를 담은 Error를
                // return합니다.
                return Err(ParsingError(parsing_table[current_state].keys().cloned().collect(), next_token));
            },
        };
    }

    // parsing이 완료되면 stack의 최상단에 저장된 node를 Tree로 wrapping하여 return합니다.
    Ok(Tree(stack.pop().unwrap().tree.unwrap()))
}

// shift와 goto를 실행하는 함수입니다.
fn shift_goto(tokens: &mut VecDeque<Node>, stack: &mut Vec<StackItem>, next_state: usize) {
    // tokens에서 token을 pop하여
    let next_token = tokens.pop_front().unwrap();
    // next state와 함께 stack에 push합니다.
    stack.push(StackItem::from(next_state, Some(next_token)));
}

// reduce를 실행하는 함수입니다.
// Reduction은 reduction rule을 나타내는 struct로, src/parser/parsing_table.rs에 정의되어 있습니다.
// Reduction의 left는 좌항의 non-terminal을, right는 우항의 non-termianl, termianl의 수를 나타내며,
// 이는 stack에서 pop해야하는 수를 의미합니다.
fn reduce(tokens: &mut VecDeque<Node>, stack: &mut Vec<StackItem>, reduction: Reduction) {
    // 해당 reduction rule에 맞는 수 만큼 stack에서 pop하여
    // children으로 push합니다.
    let mut children: Vec<Node> = vec![];
    for _ in 0..reduction.right {
        children.push(stack.pop().unwrap().tree.unwrap());
    }
    children.reverse();

    // children과 함께 non-terminal(reduction.left)을 tokens에 push합니다.
    // 이렇게 push된 non-terminal은 shift와 goto가 함께 구현된 shift_goto를 거쳐
    // 알맞게 stack으로 다시 push됩니다.
    tokens.push_front(NonTerminal(reduction.left, children));
}
