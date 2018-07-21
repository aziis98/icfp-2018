
use nmms::*;
use nmms::state::State;
use nmms::bot::Bot;

pub trait Command {
    /// Updates the state by the operation provided in the implementation
    fn execute(&self, state: &mut State);
}

/// cerca di stoppare la simulazione !!panic se non ci riesce
pub struct Halt {
    bot: Bot
}

impl Command for Halt {
    fn execute(&self, state: &mut State) {
        if self.bot.pos.x != 0 || self.bot.pos.y != 0 || self.bot.pos.y != 0 {
            panic!("non sei arrivato alla fine del percorso cazzone");
        } else if state.bots.len() > 1 {
            panic!("ci sono più di 1 bot");
        } else if state.harmonics {
            panic!("le armoniche non sono basse (sono alte == true)");
        } else {
            state.bots = Vec::new();
        }
    }
}


/// aspetta con le mani in mano
pub struct Wait {
    bot: Bot
}

impl Command for Wait {
    fn execute(&self, state: &mut State) {
        
    }
}


/// flippa le armoniche
pub struct Flip {
    bot: Bot
}

impl Command for Flip {
    fn execute(&self, state: &mut State) {
		state.harmonics = !state.harmonics;
    }
}

/// straight move
pub struct SMove {
    bot: Bot,
    lld: CoordinateDifference
}

impl Command for SMove {
    fn execute(&self, state: &mut State) {
        if self.lld.is_lld() {
            let c: Coordinate = self.bot.pos + self.lld;
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
        // state
        unimplemented!();
    }
}


pub struct LMove {
    bot: Bot,
    sld1: CoordinateDifference,
    sld2: CoordinateDifference
}

impl Command for LMove {
    fn execute(&self, state: &mut State) {
        // if !(sld1.is_sld() || sld2.is_sld()) {
        //     panic!("una delle tue variabili non è un short linear coordinate difference");
        // }
        // let c1 = bot.pos + sld1;
        // let c2 = c1 + sld2;

        // // da fare controllare che le matrici c->c1 e c1->c2 siano completamente vuote
        // // controllare che c1 e c2 siano dentro la mesh

        // bot.pos = c2;
        // state.energy += 2*sld1.mlen() + 2 + sld2.mlen();

        // state
        unimplemented!();
    }
}