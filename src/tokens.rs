pub enum Token{
    Text(String),
    Math(String),
    Newline,
}
pub enum TokenTypes{
    Text,
    Math,
    Newline,
    None,
}