#[derive(Debug, Copy, Clone)]
pub struct Trace {
    pub commands: [Command],
    pub time: i64 // A cosa serve? Per tenere conto delle step fatte? Nel caso non starebbe meglio nello State?
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub energy: u64,
    pub harmonics: bool,
    pub matrix: Matrix,
    pub bots: [Bot], // Non sarebbe meglio un Vec<Box>?
    pub trace: Trace
}