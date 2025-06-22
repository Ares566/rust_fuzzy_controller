use super::{fuzzy::{Fuzzy, new_fuzzy, gmu}, operator::{OpAnd, OpOr}};

// linguistic vars with there fuzzy values

const F_VLN:f32 = -1.0;  // Very Large Negative
pub const F_LN:f32  = -0.72; // Large Negative
const F_MN:f32  = -0.44; // Meduim Negative
const F_SN:f32  = -0.16; // Small Negative
const F_NO:f32  = 0.0;
pub const F_SP:f32  = 0.16;
const F_MP:f32  = 0.44;
const F_LP:f32  = 0.72;
const F_VLP:f32 = 1.0;

pub struct FuzzyController{
    fc_fuzzy: Fuzzy
}

pub fn new_fuzzy_controller() -> FuzzyController{

    let mut lfc_fuzzy = new_fuzzy();

   
    // all is ok
	lfc_fuzzy.add_rule(F_NO, &OpAnd{}, F_NO, F_NO);

    // the last impact does not bring results
    lfc_fuzzy.add_rule(F_VLN, &OpOr{}, F_VLN, F_VLP);
    lfc_fuzzy.add_rule(F_VLP, &OpOr{}, F_VLP, F_VLN);

    /*// we have positive effects from last impact
	lfcFuzzy.addRule(F_SN, new(F_and), F_SN, F_SP)
	lfcFuzzy.addRule(F_SP, new(F_and), F_SP, F_SN)
	lfcFuzzy.addRule(F_LN, new(F_and), F_SN, F_LP)
	lfcFuzzy.addRule(F_LP, new(F_and), F_SP, F_LN)

	// situation had become worse
	lfcFuzzy.addRule(F_LN, new(F_and), F_SP, F_LP)
	lfcFuzzy.addRule(F_LP, new(F_and), F_SN, F_LN) */
    FuzzyController{ fc_fuzzy: lfc_fuzzy}

}

impl FuzzyController {
    // Fuzzy regulator solution. Fuzzy Conclusion
// Returning impact
//
// e current error,
// de first derivative of errors
pub fn get_fuzzy_conclusion(&self, e: f32, de: f32) -> f32 {
	let mut summ_alpha_c = 0.0;
	let mut summ_alpha = 0.0;

	// Composite all Fuzzy Rules and Centriod Defuzzification
	for rule in  &self.fc_fuzzy.rules {

		let mue: f32 = gmu(e, rule.ferr);
		let mude:f32 = gmu(de, rule.fderr);

		let alpha: f32 = rule.op.ft(mue, mude);
		summ_alpha_c += alpha * rule.fthen;
		summ_alpha += alpha
	}

	return summ_alpha_c / summ_alpha
}
}
