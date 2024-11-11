use crate::{
    stringtools::StringTools,
    tokens::{Token, TokenTypes},
};

pub fn tokenize(mut s: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    s = s.replace("\r\n", "\n");
    while s.len() > 0 {
        let mut tt = TokenTypes::None;
        let mut expects = false;
        let mut because: char = ' ';
        let mut first_word = true;
        let mut wo=false;
        let st = s.remove_while(|c, _| {
            if let TokenTypes::None = tt {
                if c == '\n' {
                    tt = TokenTypes::Newline;
                    return true;
                }
                if c.is_numeric()||"+*/-^=".contains(c) {
                    tt = TokenTypes::Math;
                }
                if c.is_alphabetic() {
                    if !first_word {
                        tt = TokenTypes::Text;
                    }
                }
                if c.is_whitespace() {
                    first_word = false;
                }
            }
            match tt {
                TokenTypes::None => {}
                TokenTypes::Math => {
                    if c.is_numeric() {
                        expects = true;
                        because = c;
                        return true;
                    }
                    if "+*/-(^=".contains(c) {
                        expects = true;
                        because = c;
                        return true;
                    }
                    if c == '\n' {
                        return false;
                    }
                    if c.is_whitespace() {
                        if expects && because.is_numeric() {
                            expects = false;
                        }
                        return true;
                    }
                    if c.is_alphabetic() {
                        if expects {
                            expects = false;
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
                TokenTypes::Text => {
                    if c.is_whitespace() {
                        wo=true;
                    }else if wo{
                        return false;
                    }
                    return (c.is_whitespace() || c.is_alphabetic()) && c != '\n';
                }
                TokenTypes::Newline => {
                    return false;
                }
            }
            true
        });
        tokens.push(match tt {
            TokenTypes::Math => Token::Math(st),
            TokenTypes::Text => Token::Text(st),
            TokenTypes::Newline => Token::Newline,
            _ => Token::Text(st),
        })
    }
    tokens
}
