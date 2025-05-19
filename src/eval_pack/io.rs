use crate::parser_pack::{parser::Evaluateable, parser_obj::IO};

impl Evaluateable for IO {
    fn evaluate(&self) {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn print_self(&self) {
        println!("{:?}",self);
    }
}