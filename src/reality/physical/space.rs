// things that have to do with moving and placment
use crate::reality::body_mind::{MultipleNpcs, NpcInTransit, Npc};

use crate::reality::physical::MultipleObjects;

use std::collections::HashMap;

pub type DimensionalPlain = HashMap<Location, MultipleObjects>;
pub type Destination = isize; // should hold the Area id (placement in world vector)

type NpcTrasitWait = Vec<NpcInTransit>;

pub struct Location {
    x: isize,
    y: isize
}

pub enum Direction {

    PlusX,
    PlusY,
    NegY,
    NegX,
    Still

}

pub struct Area {

    plain: DimensionalPlain, // holds MultipleObjects at Location 
    
    max_distance: Location,
    max_npcs: isize,
    
    npcs: MultipleNpcs,
    
    npcs_output_transit: NpcTrasitWait, // hopefully ill end up implementing multithreading and these will come in handy.
    npcs_input_next: MultipleNpcs,
    
}

impl Area {

    pub fn check_new_npcs(&mut self){
        todo!("cant add npcs yet!");
    }

    pub fn delete_dead(&mut self){
        todo!("nothing can die yet!");
    }

}