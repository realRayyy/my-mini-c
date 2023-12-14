use crate::token::*;

fn is_letter(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}

pub struct TokenInfo {
    pub token_type: TokenType,
    pub token_str: Option<String>,
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input }
    }

    pub fn iter(&self) -> LexerIter {
        LexerIter::new(self.input.as_str())
    }
}

pub struct LexerIter<'a> {
    lex_str: &'a str,
    row: usize,
    column: usize,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl LexerIter<'_> {
    pub fn new(str: &str) -> LexerIter {
        let mut iter = LexerIter {
            lex_str: str,
            pos: 0,
            read_pos: 1,
            row: 1,
            column: 0,
            ch: 0,
        };
        iter.ch = str.as_bytes()[0];
        iter
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.lex_str.len() {
            self.ch = 0;
        } else {
            let temp = self.lex_str.as_bytes();
            self.ch = temp[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;

        if self.ch == b'\n' {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    pub fn match_number(&mut self) -> TokenInfo {
        let temp = self.lex_str.as_bytes();
        let pos = self.pos;

        while is_digit(temp[self.read_pos] as char) {
            self.read_char();
        }

        TokenInfo {
            token_type: TokenType::Number,
            token_str: Some(self.lex_str[pos..self.read_pos].to_string()),
        }
    }

    pub fn match_keyword(&mut self) -> TokenInfo {
        let temp = self.lex_str.as_bytes();
        let pos = self.pos;

        while is_letter(temp[self.read_pos] as char) || is_digit(temp[self.read_pos] as char) {
            self.read_char();
        }

        let str: String = self.lex_str[pos..self.read_pos].to_string();

        match &str as &str {
            "int" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Int),
                token_str: None,
            },
            "char" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Char),
                token_str: None,
            },
            "float" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Float),
                token_str: None,
            },
            "void" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Void),
                token_str: None,
            },
            "if" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::If),
                token_str: None,
            },
            "else" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Else),
                token_str: None,
            },
            "return" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::Return),
                token_str: None,
            },
            "while" => TokenInfo {
                token_type: TokenType::KeyWord(KeyWord::While),
                token_str: None,
            },
            _ => TokenInfo {
                token_type: TokenType::Var,
                token_str: Some(str),
            },
        }
    }

    fn skip_white_space(&mut self) {
        let mut ch = self.ch as char;

        while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
            ch = self.ch as char;
        }
    }
}

impl Iterator for LexerIter<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let length = self.lex_str.len();
        let temp = self.lex_str.as_bytes();

        self.skip_white_space();

        let res: Token = if self.read_pos > length {
            return None;
        } else {
            let row = self.row;
            let column = self.column;
            let token_info: TokenInfo = match self.ch as char {
                // match Operator
                '=' => {
                    if temp[self.read_pos] as char == '=' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Equal),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Assign),
                            token_str: None,
                        }
                    }
                }

                '+' => {
                    if temp[self.read_pos] as char == '+' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Inc),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Add),
                            token_str: None,
                        }
                    }
                }

                '-' => {
                    if temp[self.read_pos] as char == '-' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Dec),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Minus),
                            token_str: None,
                        }
                    }
                }

                '*' => TokenInfo {
                    token_type: TokenType::Operator(Operator::Mul),
                    token_str: None,
                },

                '/' => TokenInfo {
                    token_type: TokenType::Operator(Operator::Div),
                    token_str: None,
                },

                '<' => {
                    if temp[self.read_pos] as char == '=' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::LessEqual),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Less),
                            token_str: None,
                        }
                    }
                }

                '>' => {
                    if temp[self.read_pos] as char == '=' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::GreaterEqual),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Greater),
                            token_str: None,
                        }
                    }
                }

                '&' => {
                    if temp[self.read_pos] as char == '&' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::And),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::ILLEGAL,
                            token_str: None,
                        }
                    }
                }

                '|' => {
                    if temp[self.read_pos] as char == '|' {
                        self.read_char();
                        TokenInfo {
                            token_type: TokenType::Operator(Operator::Or),
                            token_str: None,
                        }
                    } else {
                        TokenInfo {
                            token_type: TokenType::ILLEGAL,
                            token_str: None,
                        }
                    }
                }

                // match Symbol
                '\'' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::Quote),
                    token_str: None,
                },

                '(' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::LeftParen),
                    token_str: None,
                },

                ')' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::RightParen),
                    token_str: None,
                },

                '[' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::LeftBarcket),
                    token_str: None,
                },

                ']' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::RightBarcket),
                    token_str: None,
                },

                '{' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::LeftCurly),
                    token_str: None,
                },

                '}' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::RightCurly),
                    token_str: None,
                },

                ',' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::Comma),
                    token_str: None,
                },

                ';' => TokenInfo {
                    token_type: TokenType::Symbol(Symbol::Semi),
                    token_str: None,
                },

                // match Keyword
                'a'..='z' | 'A'..='Z' => self.match_keyword(),

                // match Number
                '0'..='9' => self.match_number(),

                _ => TokenInfo {
                    token_type: TokenType::ILLEGAL,
                    token_str: None,
                },
            };

            Token {
                token_type: token_info.token_type,
                token_str: token_info.token_str,
                row,
                column,
            }
        };

        self.read_char();
        Some(res)
    }
}
