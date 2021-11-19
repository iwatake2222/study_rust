// refer to lib.rs
extern crate test_module;
use test_module::module_a;
use test_module::common_module;

// do not use lib.rs
// mod module_a;
// mod common_module;


fn main() {
    module_a::print();
    let m = common_module::common_module::CommonModule::new();
    m.print();
}
