use std::collections::HashMap;

use crate::reality::World;
use crate::reality::physical::Object;
use crate::reality::physical::space::{Direction, Destination};
use crate::reality::physical::items::{Item, ItemType};

use crate::reality::mental::Brain;


pub type MultipleNpcs = Vec<Npc>; // makes reading code easier
pub struct NpcInTransit(Destination, Npc); // for portals later

pub struct Npc {

    pub texture: Option<char>, // renders this emoji in 2d rendering
    brain: Brain,
    body: Object,
    action: Action

}

impl Npc {

    pub fn choose_action(&mut self){
        todo!("Npcs cant do actions yet"); 
    }

}

pub enum Action {

    Attack,
    Place,
    Step,
    UseItem(Item)

}