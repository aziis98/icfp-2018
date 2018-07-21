
use nmms::*;
use nmms::state::State;
use nmms::bot::*;

pub trait Command {
    /// Updates the state by the operation provided in the implementation
    fn execute(&mut self, state: &mut State);
}

/// cerca di stoppare la simulazione !!panic se non ci riesce
#[derive(Copy, Clone)]
pub struct Halt {
    bot_bid: BID
}

impl Command for Halt {
    fn execute(&mut self, state: &mut State) {

        let bot = state.find_bot(self.bot_bid);

        if bot.pos.x != 0 || bot.pos.y != 0 || bot.pos.y != 0 {
            panic!("non sei arrivato alla fine del percorso cazzone");
        } else if state.bots.len() > 1 {
            panic!("ci sono più di 1 bot");
        } else if state.harmonics {
            panic!("le armoniche non sono basse");
        } else {
            state.bots = Vec::new();
        }
    }
}


/// aspetta con le mani in mano
#[derive(Copy, Clone)]
pub struct Wait {
    bot_bid: BID
}

impl Command for Wait {
    fn execute(&mut self, state: &mut State) {
        // wait...
    }
}


/// flippa le armoniche
#[derive(Copy, Clone)]
pub struct Flip<'a> {
    bot: &'a Bot
}

impl<'a> Command for Flip<'a> {
    fn execute(&mut self, state: &mut State) {
		state.harmonics = !state.harmonics;
    }
}

/// straight move
pub struct SMove<'a> {
    bot: &'a mut Bot,
    lld: CoordinateDifference
}

impl<'a> Command for SMove<'a> {
    fn execute(&mut self, state: &mut State) {
        if !self.lld.is_lld() {
            panic!("La coordinata non è lld");
        }

        let target: Coordinate = self.bot.pos + self.lld;
        let r = Region::new(self.bot.pos, target);

        if !state.matrix.is_empty(r) {
            panic!("Ci sono cose lungo la strada");
        }

        self.bot.pos = target;
        state.energy += 2 * self.lld.mlen() as u64;

        // da fare := creo regione da punto bot.pos a c e controllo che sia completamente vuota
        // let testing = Matrix::new()
        // if testing.is_empty {
        //  buono --> voglio spostare il bot
        //  state.energy += 2*lld.mlen()
        //  bot.pos = c
        // } else {
        //  panic!("non è completamente vuota la roba che stai cercando di attraversare!!!!!")
        // }
        // state
    }
}

pub struct LMove<'a> {
    bot: &'a mut Bot,
    sld1: CoordinateDifference,
    sld2: CoordinateDifference
}

impl<'a> Command for LMove<'a> {
    fn execute(&mut self, state: &mut State) {

        if !self.sld1.is_sld() {
            panic!("Il primo parametro non è una sdl");
        }

        if !self.sld2.is_sld() {
            panic!("Il secondo parametro non è una sdl");
        }

        let c1 = self.bot.pos + self.sld1;
        let c2 = c1 + self.sld2;

        let r1 = Region::new(self.bot.pos, c1);
        let r2 = Region::new(c1, c2);

        if !state.matrix.is_empty(r1) {
            panic!("Il primo pezzo non è vuoto");
        }

        if !state.matrix.is_empty(r2) {
            panic!("Il secondo pezzo non è vuoto");
        }

        self.bot.pos = c2;
        state.energy += 2 * (self.sld1.mlen() + 2 + self.sld2.mlen()) as u64;

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
    }
}


pub struct Fission<'a> {
    bot: &'a mut Bot,
    nd: CoordinateDifference,
    m: u32
}

impl<'a> Command for Fission<'a> {
    fn execute(&mut self, state: &mut State) {
        if self.bot.seeds.len() == 0 {
            panic!();
        }

        let c1 = self.bot.pos + self.nd;

        if state.matrix.get_voxel(c1) {
            panic!();
        }

        if (self.bot.seeds.len() as u32) < self.m + 1 {
            panic!();
        }

        let mut new_seeds: Vec<u32> = Vec::new();

        for _ in 0 .. self.m {
            new_seeds.push(self.bot.seeds.remove(1));
        }

        let new_bot = Bot {
            bid: self.bot.seeds.remove(0),
            pos: c1,
            seeds: new_seeds
        };

        state.bots.push(new_bot);
        state.energy += 24;
    }
}


pub struct Fill<'a> {
    bot: &'a mut Bot,
    nd: CoordinateDifference
}
impl<'a> Command for Fill<'a> {
    fn execute(&mut self, state: &mut State) {
        if !self.nd.is_nd() {
            panic!();
        }

        let c1 = self.bot.pos + self.nd;

        if !state.matrix.get_voxel(c1) {
            state.matrix.set_voxel(c1, true);
            state.energy += 12;
        }
        else {
            state.energy += 6;
        }
    }
}


pub struct Void<'a> {
    bot: &'a mut Bot,
    nd: CoordinateDifference
}
impl<'a> Command for Void<'a> {
    fn execute(&mut self, state: &mut State) {
        if !self.nd.is_nd() {
            panic!();
        }

        let c1 = self.bot.pos + self.nd;

        if state.matrix.get_voxel(c1) {
            state.matrix.set_voxel(c1, false);
            state.energy -= 12;
        }
        else {
            state.energy += 6;
        }
    }
}


pub struct FusionP<'a> {
    bot: &'a mut Bot,
    nd: CoordinateDifference
}
impl<'a> Command for FusionP<'a> {
    fn execute(&mut self, state: &mut State) {

    }
}