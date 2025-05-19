use crate::parser_pack::{parser::Evaluateable, parser_obj::Movement};

impl Evaluateable for Movement {
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