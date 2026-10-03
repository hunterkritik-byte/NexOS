pub mod gui;
pub mod runtime;

pub struct AppRegistry {
    count: usize,
}

impl AppRegistry {
    pub const fn new() -> Self { Self { count: 0 } }
    pub fn count(&self) -> usize { self.count }
    pub fn register(&mut self) { self.count += 1; }
}
