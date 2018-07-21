
use nmms::Matrix;
use nmms::bot::Bot;
use nmms::bot::commands::Command;

pub struct Trace<'a> {
    pub commands: Vec<&'a Command>,
    pub time: i64 // A cosa serve? Per tenere conto delle step fatte? Nel caso non starebbe meglio nello State?
}

pub struct State<'a> {
    pub energy: u64,
    pub harmonics: bool,
    pub matrix: Matrix,
    pub bots: Vec<Bot>,
    pub trace: Trace<'a>
}