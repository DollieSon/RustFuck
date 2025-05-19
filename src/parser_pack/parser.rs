use std::{any::Any};
use crate::{lexer_pack::lex_tokens::{LextToken, TokenType}};
use super::{parser_errors::ParserErrors, parser_obj::{Block, IOType, MoveDir, Movement, Operation, IO}};

pub trait Evaluateable {
    fn evaluate(&self);
    fn as_any(&self) -> &dyn Any;
    fn print_self(&self);
}

pub struct Parser{
    tokens:Vec<LextToken>,
    curr_index:usize
}

impl Parser{
    pub fn new(lex_tok:Vec<LextToken>) -> Self{
        Parser { 
            tokens: lex_tok,
            curr_index:0
        }
    }

    pub fn evaluate(&mut self) ->Block{
        let mut res = Block{
            instructions:vec![]
        };
        let mut temp_res = Block{
            instructions:vec![]
        };
        let mut bracket_stack = Vec::<&LextToken>::new();
        while let Some(tok) = self.tokens.get(self.curr_index) {
            match tok.token {
                TokenType::MOVE_L | TokenType::MOVE_R => {
                    let res_token = self.parse_movement();
                    match res_token {
                        Ok(clean_tok) => {
                            temp_res.instructions.push(Box::new(clean_tok));
                        }
                        Err(err) => {
                            panic!("ParsingErr: Movement - {:?}",err);
                        }
                    }
                }
                TokenType::DECREMENT | TokenType::INCREMENT =>{
                    let res_token = self.parse_operation();
                    match res_token {
                        Ok(clean_tok) => {
                            temp_res.instructions.push(Box::new(clean_tok));
                        }
                        Err(err) => {
                            panic!("ParsingErr: Movement - {:?}",err);
                        }
                    }
                }
                TokenType::INPUT | TokenType::OUTPUT => {
                    let res_token = self.parse_IO();
                    match res_token {
                        Ok(tok) =>{
                            temp_res.instructions.push(Box::new(tok));
                        }
                        Err(err)=>{
                            panic!("ParsingErr: Input - {:?}",err);
                        }
                    }
                }
                TokenType::BRACKET_O => {
                    res.instructions.append(&mut temp_res.instructions);
                }
                TokenType::BRAKET_C => {
                    res.instructions.push(Box::new(temp_res));
                    temp_res = Block{
                        instructions:vec![]
                    }
                }
                _ => {
                    panic!("Parsing Erro: Uncovered token found");
                }
            } 
            self.curr_index+=1;
        }
        if temp_res.instructions.len() != 0 {
            res.instructions.append(&mut temp_res.instructions);
        }
        //tempoarary
        return res;
    }
    // fn peek_token(&self) -> Result<&LextToken,ParserErrors>{
    //     if let Some(x) = self.tokens.get(self.curr_index+1){
    //         return Ok(x);
    //     }else{
    //         return Err(ParserErrors::NoPeekToken);
    //     }
    // }
    fn get_next_token_if_eq(&mut self,tok_type:&TokenType)-> Result<&LextToken,ParserErrors>{
        if let Some(x) = self.tokens.get(self.curr_index){
            if x.token == *tok_type {
                self.curr_index+=1;
                return Ok(x);
            }else{
                return Err(ParserErrors::TokenTypeNotEq);
            }
        }else{
            return Err(ParserErrors::NoPeekToken);
        }
    }
    // assume that current token is a movement token
    fn parse_movement(&mut self)->Result<Movement,ParserErrors>{
        let move_dir = self.tokens.get(self.curr_index).expect("IndexOutOfBounds").token.clone();
        let mut res:Result<Movement, ParserErrors> =Ok(Movement { steps: 1, direction: if move_dir == TokenType::MOVE_L {MoveDir::Left} else {MoveDir::Right} });
        while let Ok(_) = self.get_next_token_if_eq(&move_dir){
            match res{
                Ok(mut temp) => {
                    temp.steps+=1;
                    res = Ok(temp)
                }
                Err(err) =>{
                    return Err(err);
                }
            }
        }
        return res;
    }
    //assume that current token is a operation
    fn parse_operation(&mut self) -> Result<Operation,ParserErrors>{
        let mut oper:Result<Operation,ParserErrors> = Err(ParserErrors::UnexpectedReturn);
        let numer = self.tokens.get(self.curr_index).expect("IndexOutOfBoundsError").token.clone();
        while let Ok(_) = self.get_next_token_if_eq(&numer){
            match oper {
                Ok(mut op)=>{
                    op.value+= if numer == TokenType::DECREMENT {-1} else {1};
                    oper = Ok(op);
                }
                Err(_) => {
                    oper = Ok(Operation { value:if numer == TokenType::DECREMENT {-1} else {1} })
                }
            }
        }
        return oper;
    }
    fn parse_IO(&mut self)->Result<IO,ParserErrors>{
        let io_type:TokenType = self.tokens.get(self.curr_index).expect("IndexOutOfBoundsErr").token.clone();
        return Ok(IO{
            IO_type:if(io_type == TokenType::INPUT) {IOType::Input} else {IOType::Output}
        });
    }

}