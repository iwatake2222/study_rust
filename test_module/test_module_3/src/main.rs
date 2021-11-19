// use lib.rs
// extern crate test_module;
// use test_module::module_a;
// use test_module::module_b;

// do not use lib.rs
mod module_a;
mod module_b;


fn main() {
    module_a::print();
    module_b::module_b_sub_0::print();
}
