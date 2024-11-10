use crate::{mathcompiler::math, state::State};

pub fn compile(mut s: String) -> String {
    let mut o = String::new();
    s=s.replace("\r\n", "\n");
    let mut chars = s.chars().peekable();
    let mut state: State = State::None;
    while let Some(c) = chars.next() {
        loop {
            match state {
                State::Text => {
                    if c == '\n' {
                        state = State::None;
                        o = format!("{o}\n");
                        break;
                    } else if c.is_numeric() {
                        state = State::Smath;
                        o = format!("{o}$");
                    } else {
                        o = format!("{o}{c}");
                        break;
                    }
                }
                State::Bmath => {
                    if c == '\n' {
                        state = State::None;
                        o = format!("{o}$$\n");
                        break;
                    } else {
                        o = math(o, c);
                        break;
                    }
                }
                State::Smath => {
                    if c == '\n' {
                        state = State::None;
                        o = format!("{o}$\n");
                        break;
                    } else if c.is_alphabetic() {
                        let next = chars.peek();
                        if let Some(n) = next {
                            if n.is_alphabetic() {
                                state = State::Text;
                                o = format!("{o}$");
                            } else {
                                o = math(o, c);
                                break;
                            }
                        } else {
                            o = math(o, c);
                            break;
                        }
                    } else {
                        o = math(o, c);
                        break;
                    }
                }
                State::None => {
                    if c.is_whitespace() {
                        o = format!("{o}{c}");
                        break;
                    }
                    if c.is_alphabetic() {
                        let next = chars.peek();
                        if let Some(n) = next {
                            if n.is_alphabetic() {
                                state = State::Text;
                            } else {
                                state = State::Bmath;
                                o = format!("{o}$$");
                            }
                        } else {
                            o = format!("{o}{c}");
                            break;
                        }
                    } else {
                        state = State::Bmath;
                        o = format!("{o}$$");
                    }
                }
            }
        }
    }
    match state {
        State::Bmath => {
            o=format!("{o}$$");
        },
        State::Smath => {
            o=format!("{o}$");
        },
        _=>{}
    }
    o=format!("\\documentclass{{article}}\n\\usepackage{{amssymb}}\n\\usepackage{{amsmath}}\n\\begin{{document}}\n{o}\n\\end{{document}}");
    o
}
