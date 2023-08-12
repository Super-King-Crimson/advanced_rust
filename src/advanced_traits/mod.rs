pub mod associated_types;
pub mod operator_overloading;
pub mod method_disambiguation;

pub fn explain() {
    println!("Once again, we've made it. MORE TRAITS!");

    associated_types::explain();
    operator_overloading::explain();
    method_disambiguation::explain();
}