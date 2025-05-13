use core::fmt;


pub enum TokenType{
    MOVE_L,MOVE_R,INCREMENT,DECREMENT,OUTPUT,INPUT,BRACKET_O,BRAKET_C
}

pub struct LextToken{
    pub token:TokenType,
    pub line_num:i32,
}

impl LextToken{
    pub fn new(token_type:TokenType,line_num:i32) -> LextToken{
        return LextToken{
            token:token_type,
            line_num:line_num
        }
    }
    pub fn get_token(ch:char,line:i32) -> LextToken{
        match ch  {
            '>' => {
               return Self::new(TokenType::MOVE_R, line);
            },
            '<' => {
               return Self::new(TokenType::MOVE_L, line);
            },
            '-' => {
               return Self::new(TokenType::DECREMENT, line);
            },
            '+' => {
               return Self::new(TokenType::INCREMENT, line);
            },
            '.' => {
               return Self::new(TokenType::MOVE_R, line);
            },
            ',' => {
               return Self::new(TokenType::OUTPUT, line);
            },
            '[' => {
               return Self::new(TokenType::BRACKET_O, line);
            },
            ']' => {
               return Self::new(TokenType::BRAKET_C, line);
            },
            _ => {
                panic!("Error character not registered");
            }
        }
    }
}