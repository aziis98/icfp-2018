
use std::*;

use nmms::Matrix;
use nmms::bot::*;
use nmms::bot::commands::Command;

type Trace = Vec<Box<Command>>;

pub struct State {
    pub energy: u64,
    pub harmonics: bool,
    pub matrix: Matrix,
    pub bots: Vec<Bot>, // TOOO: Meglio se fosse una LinkedHashMap<BID, Bot>
    pub trace: Trace
}

impl State {
    fn new(m: Matrix, program: Trace) -> State {
        State {
            energy: 0,
            harmonics: false,
            matrix: m,
            bots: Vec::new(),
            trace: program
        }
    }

    pub fn find_bot(&self, bot_bid: BID) -> Box<Bot> {
        unimplemented!();
    }
}