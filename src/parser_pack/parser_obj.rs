use super::parser::Evaluateable;
#[derive(Debug)]
pub enum MoveDir {
    Right,
    Left
}
#[derive(Debug)]
pub enum IOType{
    Input,
    Output
}
#[derive(Debug)]
pub struct Movement {
    pub steps:usize,
    pub direction:MoveDir
}
#[derive(Debug)]
pub struct Operation{
    pub value:i32,
}
#[derive(Debug)]
pub struct IO{
    pub IO_type: IOType
}
pub struct Block{
    pub instructions:Vec<Box<dyn Evaluateable>>
}