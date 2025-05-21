use std::{any::Any, thread::panicking};
use crate::{lexer_pack::lex_tokens::{LextToken, TokenType}};
use super::{parser_errors::ParserErrors, parser_obj::{Block, IOType, MoveDir, Movement, Operation, IO}};

pub trait Evaluateable {
    fn evaluate(&self);
    fn as_any(&self) -> &dyn Any;
    fn print_self(&self);
}

pub struct Parser{
    tokens:Vec<LextToken>,
    curr_index:usize,
    block_stack:Vec<Block>
}

impl Parser{
    pub fn new(lex_tok:Vec<LextToken>) -> Self{
        Parser { 
            tokens: lex_tok,
            curr_index:0,
            block_stack:vec![]
        }
    }

    pub fn evaluate(&mut self) ->&Block{
        let temp_block = Block{
            instructions:vec![]
        };
        self.block_stack.push(temp_block);
        while let Some(tok) = self.get_next_token() {
            // println!("Token :{:?}",tok.token);
            match tok.token {
                TokenType::MOVE_L =>{
                    // should probably be a macro
                    let mut res_token = Movement{
                        steps: 1,
                        direction: MoveDir::Left
                    };
                    while let Ok(_) = self.get_next_token_if_eq(&TokenType::MOVE_L)  {
                        res_token.steps+=1;
                    }
                    self.insert_to_top(res_token);
                }
                TokenType::MOVE_R => {
                    let mut res_token = Movement{
                        steps: 1,
                        direction: MoveDir::Right
                    };

                    while let Ok(_) = self.get_next_token_if_eq(&TokenType::MOVE_R)  {
                        res_token.steps+=1;
                    }
                    self.insert_to_top(res_token);
                }
                TokenType::DECREMENT => {
                    let mut res_token =  Operation{
                        value:-1,
                    };
                    while let Ok(_) = self.get_next_token_if_eq(&TokenType::DECREMENT)  {
                        res_token.value-=1;
                    }
                    self.insert_to_top(res_token);
                } 
                TokenType::INCREMENT =>{
                    let mut res_token =  Operation{
                        value:1,
                    };
                    while let Ok(_) = self.get_next_token_if_eq(&TokenType::INCREMENT)  {
                        res_token.value+=1;
                    }
                    self.insert_to_top(res_token);
                }
                // should not exist btw
                TokenType::INPUT => {
                    let res_token =  IO{
                        IO_type:IOType::Input
                    };
                    self.insert_to_top(res_token);
                }
                TokenType::OUTPUT => {
                    let res_token =  IO{
                        IO_type:IOType::Output
                    };
                    self.insert_to_top(res_token);
                }
                TokenType::BRACKET_O => {
                    let res_token = Block{
                        instructions:vec![]
                    };
                    self.block_stack.push(res_token);
                    // self.insert_to_top(res_token);
                }
                //TODO: handle errors
                TokenType::BRAKET_C => {
                    if self.block_stack.len() <= 1 {
                        panic!("Parsing Error: Closing Bracket without pair");
                    }
                    let top = self.block_stack.pop().unwrap();
                    self.insert_to_top(top);
                }
                _ => {
                    panic!("Parsing Error: Uncovered token found");
                }
            } 
        }
        if self.block_stack.len() > 1 {
            panic!("Parsing Error: Unclosed Bracket")
        }
        //tempoarary
        return self.block_stack.get(0).unwrap();
    }
    fn get_next_token(&mut self) ->Option<&LextToken>{
        let res = self.tokens.get(self.curr_index);
        self.curr_index+=1;
        return res;
    }
    fn peek_token(&mut self)->Option<&LextToken>{
        return self.tokens.get(self.curr_index);
    }
    fn get_next_token_if_eq(&mut self,tok_type:&TokenType)-> Result<&LextToken,ParserErrors>{
        if let Some(x ) = self.peek_token(){
            if x.token == *tok_type{
                // println!("Tok Eq");
                return Ok(self.get_next_token().unwrap());
            }else{
                // println!("Tok not Eq");
                return Err(ParserErrors::TokenTypeNotEq);
            }
        }else{
            // println!("Peek Not allowed");
            return Err(ParserErrors::NoPeekToken);
        }
    }
    fn insert_to_top(&mut self,instruction : impl Evaluateable + 'static){
        let topmost = self.block_stack.last_mut().expect("Error Block Stack Empty");
        topmost.instructions.push(Box::new(instruction));
    }

}