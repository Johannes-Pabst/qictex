use crate::tokens::Token;

pub fn parse(t:Vec<Token>)->String {
    let mut s=String::new();
    let mut i=0;
    while i<t.len(){
        match &t[i]{
            Token::Text(text)=>s.push_str(&text),
            Token::Math(math)=>{
                let mut front=false;
                let mut back=false;
                if i==0{
                    front=true;
                }else{
                    if let Token::Newline=t[i-1]{
                        front=true;
                    }
                }
                if i==t.len()-1{
                    back=true;
                }else{
                    if let Token::Newline=t[i+1]{
                        back=true;
                    }
                }
                // TODO: parse math str
                if front&&back{
                    s.push_str(&format!("$${}$$",math));
                }else{
                    s.push_str(&format!("${}$",math));
                }
            },
            Token::Newline=>{
                s.push('\n');
            }
        }
        i+=1;
    }
    s=format!("\\documentclass{{article}}\n\\usepackage{{amssymb}}\n\\usepackage{{amsmath}}\n\\begin{{document}}\n{s}\n\\end{{document}}");
    s
}