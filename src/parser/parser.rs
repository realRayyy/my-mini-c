use crate::token::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AstNode {
    ast_type: String,
    children: RefCell<Vec<Rc<AstNode>>>,
}
pub struct Parser<'a> {
    token_list: &'a Vec<Token>,
    pos: usize,
    ast_root: Rc<AstNode>,
}

impl Parser<'_> {
    pub fn new(token_list: &Vec<Token>) -> Parser {
        Parser {
            token_list,
            pos: 0,
            ast_root: Rc::new(AstNode {
                ast_type: String::from("program"),
                children: RefCell::new(vec![]),
            }),
        }
    }

    pub fn next(&mut self) {
        self.pos += 1;
    }

    pub fn match_token(&mut self, token_type: TokenType) {
        if self.token_list[self.pos].token_type == token_type {
            self.next();
        }
    }

    pub fn program(&mut self) {
        while self.pos < self.token_list.len() {
            self.declaration(&self.ast_root.clone());
        }
    }

    pub fn declaration(&mut self, ast_node: &Rc<AstNode>) {
        let node = Rc::new(AstNode {
            ast_type: String::from("declaration"),
            children: RefCell::new(vec![]),
        });

        self.type_token(&node.clone());

        ast_node.children.borrow_mut().push(node);
    }

    pub fn type_token(&mut self, ast_node: &Rc<AstNode>) {
        match self.token_list[self.pos].token_type {
            TokenType::KeyWord(KeyWord::Void) => {}

            _ => {}
        };
    }
}
