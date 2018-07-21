
use nmms::Coordinate;

pub mod commands;

#[derive(Debug)]
pub struct Bot {
    pub bid: i32,
    pub pos: Coordinate,
    pub seeds: Vec<i32> // Non sarebbe meglio un Vec<i32> ?
}