mod module_a {
    pub fn print() {
        println!("I'm moule_a");
    }
}

mod module_b {
    pub fn print() {
        println!("I'm moule_b");
    }
}

fn main() {
    module_a::print();
    module_b::print();
}
