pub enum TokenType{
    MOVE_L,MOVE_R,INCREMENT,DECREMENT,OUTPUT,INPUT,BRACKET_O,BRAKET_C
}

pub struct LextToken{
    pub token:TokenType,
    pub line_num:u32,
}

impl LextToken{
    pub fn new(token_type:TokenType,line_num:u32) -> LextToken{
        return LextToken{
            token:token_type,
            line_num:line_num
        }
    }

}