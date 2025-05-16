use crate::parser_pack::{parser::Evaluateable, parser_obj::Operation};

impl Evaluateable for Operation {
    fn evaluate(&self) {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}