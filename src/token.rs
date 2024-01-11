#[derive(Debug)]
pub enum KeyWord {
    Int,
    Char,
    Float,
    Void,
    If,
    Else,
    Return,
    While,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Inc, // ++
    Minus,
    Dec, // --
    Mul,
    Div,
    Assign,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum Symbol {
    Semi,         // ;
    Comma,        // ,
    LeftParen,    // (
    RightParen,   // )
    LeftBarcket,  // [
    RightBarcket, // ]
    LeftCurly,    // {
    RightCurly,   // }
    Quote,        // '
}

#[derive(Debug)]
pub enum TokenType {
    KeyWord(KeyWord),
    Operator(Operator),
    Symbol(Symbol),
    Var,
    Number,
    ILLEGAL,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub token_str: Option<String>,
    pub row: usize,
    pub column: usize,
}
