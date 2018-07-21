#[derive(Debug, Copy, Clone)]
pub enum Command {
    // list of possible commands
    halt,
    wait,
    flip,
    smove,
    lmove,
    fission,
    fill
}

pub struct Trace {
    pub commands: [Command],
    pub time: i64
}



#[derive(Debug, Copy, Clone)]
pub struct S {
    pub energy: u64,
    pub matrix: Matrix,
    pub bots: [Bot],
    pub trace: Trace
}