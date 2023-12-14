use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use mini_c::lexer::lexer::*;
use mini_c::token::*;

fn main() -> Result<(), Error> {
    let path = "example/test.c";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut token_list: Vec<Token> = Vec::new();

    let mut row = 0;
    for line in buffered.lines() {
        let mut input_str: String = String::from(line?);
        input_str.push(' ');
        let lexer = Lexer::new(input_str);

        row += 1;
        for mut token in lexer.iter() {
            token.row = row;
            println!(
                " -> type: {:?} | value: {:?} | row: {:?} | column: {:?}",
                token.token_type, token.token_str, token.row, token.column
            );
            token_list.push(token);
        }
    }

    Ok(())
}
