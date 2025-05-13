use crate::lexer_pack::lex_tokens::LextToken;
pub struct Lexer{
    pub line:String,
}
impl Lexer{
    pub fn new(lines:String) -> Lexer{
        Lexer{
            line:lines
        }
    }
    pub fn analize(&mut self) -> Vec<LextToken>{
        let mut res = vec![];
        for (ind,ch) in self.line.chars().enumerate(){
            res.push(LextToken::get_token(ch, ind as i32));
        }
        return res;
    }
}