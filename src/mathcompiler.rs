
pub fn math(s:String,c:char)->String{
    format!("{s}{}",match c {
        '*'=>"\\cdot ".to_string(),
        '+'=>"+".to_string(),
        '-'=> "-".to_string(),
        '/'=> "/".to_string(),
        '('=> "\\left(".to_string(),
        ')'=> "\\right)".to_string(),
        '['=> "\\left[".to_string(),
        ']'=> "\\right]".to_string(),
        '€'=> "\\in".to_string(),
        _=> c.to_string(),
    })
}