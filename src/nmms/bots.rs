#[derive(Debug, Copy, Clone)]
pub struct Bot {
    pub bid: i32,
    pub pos: Coordinate,
    pub seeds: [i32]
}

pub trait Command {
    fn execute(&self, state: State) -> State;
}


pub struct Halt { // cerca di stoppare la simulazione !!panic se non ci riesce
    bot: Bot
}
impl Command for Halt {
    fn execute(&self, state: State)-> State{
        if bot.pos.x != 0 || bot.pos.y != 0 || bot.pos.y != 0 {
            panic!("non sei arrivato alla fine del percorso cazzone");
        } else if state.bots.len() > 1 {
            panic!("ci sono più di 1 bot");
        } else if state.harmonics {
            panic!("le armoniche non sono basse (sono alte == true)");
        } else {
            state.bots: Bot = Vec::new();
            return state
        }
    }
}


pub struct Wait { // aspetta con le mani in mano
    bot: Bot
}
impl Command for Wait {
    fn execute(&self, state: State)-> State{
        return state
    }
}


pub struct Flip { // flippa le armoniche
    bot: Bot
}
impl Command for Flip {
    fn execute(&self, state: State)-> State{
        if state.harmonics {
            state.harmonics = false
        } else {
            state.harmonics = true
        }
        return state
    }
}


pub struct SMove { // straight move
    bot: Bot,
    lld: CoordinateDifference
}
impl Command for SMove {
    fn execute(&self, state: State)-> State{
        if lld.is_lld() {
            let c: CoordinateDifference = bot.pos + lld;

        }
    }
}