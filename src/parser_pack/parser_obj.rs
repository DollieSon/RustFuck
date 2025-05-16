use super::parser::Evaluateable;

pub enum MoveDir {
    Right,
    Left
}
pub struct Movement {
    pub steps:usize,
    pub direction:MoveDir
}

pub struct Operation{
    pub value:i32,
}

pub enum IOType{
    Input,
    Output
}
pub struct IO{
    pub IO_type: IOType
}

pub struct Block{
    pub instructions:Vec<Box<dyn Evaluateable>>
}