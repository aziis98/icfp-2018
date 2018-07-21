#[derive(Debug, Copy, Clone)]
pub struct Bot {
    pub bid: i32,
    pub pos: Coordinate,
    pub seeds: [i32]
}

impl Bot {
    pub fn halt(&self, S) -> bool {
        if self.pos == 0 && self.pos == 0 && self.pos == 0 {
            false
        }
        else if S.armonics == false {
            false
        }

        let mut S.bots = {};
    }
}