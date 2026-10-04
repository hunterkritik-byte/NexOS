pub struct TerminalState {
    pub columns: usize,
    pub rows: usize,
}
impl TerminalState {
    pub const fn new(columns: usize, rows: usize) -> Self { Self { columns, rows } }
}
