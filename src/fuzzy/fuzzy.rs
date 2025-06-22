use crate::fuzzy::rule;
use heapless::Vec;
use num_traits::*;

pub struct Fuzzy {
    pub rules: Vec<rule::Rule, 10>,
}
pub fn new_fuzzy() -> Fuzzy {
    Fuzzy { rules: Vec::new() }
}
impl Fuzzy {
    pub fn add_rule(
        &mut self,
        ferr: f32,
        op: &'static dyn  super::operator::Operator,
        fderr: f32,
        fthen: f32,
    ) {
        let new_rule = rule::new_rule(ferr, op, fderr, fthen);
        _ = self.rules.push(new_rule);
    }

    
}
pub fn gmu(x: f32, a: f32) -> f32 {
    (-((x - a).powi(2) / (2.0 * (0.3_f32).powi(2)))).exp()
}