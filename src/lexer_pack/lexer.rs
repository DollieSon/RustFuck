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
        for ch in self.line.chars(){
            println!("{ch}");
        }
        return res;
    }
}