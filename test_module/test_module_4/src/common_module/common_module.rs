pub struct CommonModule {
    id: i32,
}

impl CommonModule {
    pub fn new() -> CommonModule {
        CommonModule {
            id: 123,
        }
    }
    pub fn print(&self) {
        println!("id = {}", self.id);
    }
}
