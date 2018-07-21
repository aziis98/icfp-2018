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
    fn execute(&self, state: State) -> State {
        if lld.is_lld() {
            let c: CoordinateDifference = bot.pos + lld;
            // da fare := creo matrice da punto bot.pos a c e controllo che sia completamente vuota
            // let testing = Matrix::new()
            // if testing.is_empty {
            //  buono --> voglio spostare il bot
            //  state.energy += 2*lld.mlen()
            //  bot.pos = c
            // } else {
            //  panic!("non è completamente vuota la roba che stai cercando di attraversare!!!!!")
            // }
        }
        state
    }
}


pub struct LMove {
    bot: Bot,
    sld1: CoordinateDifference,
    sld2: CoordinateDifference
}
impl Command for LMove {
    fn execute(&self, state: State) -> State {
        if !(sld1.is_sld() || sld2.is_sld()) {
            panic!("una delle tue variabili non è un short linear coordinate difference");
        }
        let c1 = bot.pos + sld1;
        let c2 = c1 + sld2;

        // da fare controllare che le matrici c->c1 e c1->c2 siano completamente vuote
        // controllare che c1 e c2 siano dentro la mesh

        bot.pos = c2;
        state.energy += 2*sld1.mlen() + 2 + sld2.mlen();

        state
    }
}