use crate::fuzzy::operator;


pub struct Rule {
    pub ferr: f32,
    pub op: &'static dyn operator::Operator,
    pub fderr: f32,
    pub fthen: f32,
}

pub fn new_rule(_ferr: f32, _op: &'static dyn operator::Operator, _fderr: f32, _fthen: f32) -> Rule {
    Rule {
        ferr: _ferr,
        op: _op,
        fderr: _fderr,
        fthen: _fthen,
    }
}
