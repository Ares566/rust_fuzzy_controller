pub mod fuzzy;

fn main() {
    let fuzzy_cntl = fuzzy::controller::new_fuzzy_controller();
    let concl = fuzzy_cntl.get_fuzzy_conclusion(fuzzy::controller::F_LN, fuzzy::controller::F_SP);
    println!("{:?}", concl)
}
