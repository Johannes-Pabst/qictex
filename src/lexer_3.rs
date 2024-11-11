pub enum WType {
    Word,
    Letter,
    Number,
    BinOp,
    UnOp,
    Unknown,
    TexComm,
    WhiteSpace,
    NewLine,
}
pub fn tokenize(mut s: String) -> Vec<(WType, String)> {
    s = s.replace("\r\n", "\n");
    let mut cur_type: WType = WType::Unknown;
    let mut cur_str: String = String::new();
    let mut chars = s.chars().peekable();
    let mut v = Vec::new();
    let bin_ops = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string(),
        "=".to_string(),
        "!=".to_string(),
        ">".to_string(),
        "<".to_string(),
        "=>".to_string(),
        "<=".to_string(),
        "<=>".to_string(),
        "€".to_string(),
        "%".to_string(),
        "&".to_string(),
        "|".to_string(),
    ];
    let un_ops = vec!["!".to_string()];
    while let Some(c) = chars.next() {
        loop {
            let b = match cur_type {
                WType::Word => c.is_alphabetic(),
                WType::TexComm => c.is_alphabetic(),
                WType::Number => c.is_numeric() || c == '.',
                WType::BinOp => {
                    if contains_sq(&cur_str, &bin_ops) {
                        true
                    } else if contains_sq(&cur_str, &un_ops) {
                        cur_type = WType::UnOp;
                        true
                    } else {
                        false
                    }
                },
                WType::NewLine=>{
                    false
                },
                WType::WhiteSpace=>{
                    c!='\n'&&c.is_whitespace()
                }
                WType::Unknown => {
                    if c.is_alphabetic() {
                        cur_type = WType::Word;
                        true
                    } else if c.is_numeric() {
                        cur_type = WType::Number;
                        true
                    } else if c == '\\' {
                        cur_type = WType::TexComm;
                        true
                    } else if "><=|&+*-/%!€".contains(c) {
                        cur_type = WType::BinOp;
                        true
                    }else if c=='\n'{
                        cur_type=WType::NewLine;
                        true
                    }else if c.is_whitespace(){
                        cur_type=WType::WhiteSpace;
                        true
                    } else {
                        panic!();
                    }
                }
                _ => {
                    panic!();
                }
            };
            if b {
                cur_str.push(c);
                break;
            }else{
                v.push((cur_type, cur_str.clone()));
                cur_type = WType::Unknown;
                cur_str.clear();
            }
        }
    }
    return v;
}
pub fn contains_sq(s: &String, v: &Vec<String>) -> bool {
    return v
        .iter()
        .map(|x| x.starts_with(s))
        .collect::<Vec<bool>>()
        .contains(&true);
}
