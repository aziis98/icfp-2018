
use nmms::Coordinate;

pub mod commands;

pub type BID = u32;

#[derive(Debug, Clone)]
pub struct Bot {
    pub bid: BID,
    pub pos: Coordinate,
    pub seeds: Vec<BID> // Questo è ok anche Vec, non serve che sia altro
}

