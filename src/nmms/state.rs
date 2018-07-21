#[derive(Debug, Copy, Clone)]
pub struct Trace {
    pub commands: [Command],
    pub time: i64
}


#[derive(Debug, Copy, Clone)]
pub struct State {
    pub energy: u64,
    pub harmonics: bool,
    pub matrix: Matrix,
    pub bots: [Bot],
    pub trace: Trace
}