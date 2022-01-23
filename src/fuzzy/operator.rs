pub trait Operator {
    fn ft(&self,op1: f32, op2: f32) -> f32;
}

pub struct OpAnd {}
impl Operator for OpAnd {
    fn ft(&self, op1: f32, op2: f32) -> f32 {
        f32::min(op1, op2)
    }
}
pub struct OpOr {}
impl Operator for OpOr {
    fn ft(&self, op1: f32, op2: f32) -> f32 {
        f32::max(op1, op2)
    }
}
