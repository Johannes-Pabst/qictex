use crate::tokens::{Token, TokenTypes};

pub fn tokenize(mut s: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    s = s.replace("\r\n", "\n");
    
    tokens
}
