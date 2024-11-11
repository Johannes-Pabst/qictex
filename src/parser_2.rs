use crate::lexer_3::WType;

pub enum TType {
    Word,
    Variable,
    Number,
    BinOp,
    UnOp,
    TexComm,
}
pub fn parse(t:Vec<(WType, String)>)->Vec<(TType, String)>{
    let v=Vec::new();

    v
}