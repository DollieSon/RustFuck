use std::any::Any;

use crate::lexer_pack::lex_tokens::LextToken;

use super::parser_obj::{Block, Operation};

pub trait Evaluateable {
    fn evaluate(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct Parser{
    tokens:Vec<LextToken>,
}

impl Parser{
    pub fn new(lex_tok:Vec<LextToken>) -> Self{
        Parser { tokens: lex_tok }
    }

    pub fn evaluate(&self) ->Block{
        let mut res = Block{
            instructions:vec![]
        };
        let mut bracket_stack = Vec::<&LextToken>::new();
        let mut curr_tok:Box<dyn Evaluateable> = Box::new(Operation{
            value:0
        });
        //tempoarary
        return res;
        }
}