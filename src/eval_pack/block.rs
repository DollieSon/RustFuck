use crate::{parser_pack::{parser::Evaluateable, parser_obj::{Block, Movement}}};

impl Evaluateable for Block {
    fn evaluate(&self) {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn print_self(&self){
        println!("Block :[ {}", self.instructions.len());
        for thing in self.instructions.iter(){
            thing.print_self();
        }
        println!("]\n");
    }

}

impl Block {
}