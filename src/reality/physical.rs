// the solid world, blocks and bodies

pub mod placement;
pub mod items;

use crate::reality::physical::placement::{Direction, Destination, Location};
use crate::reality::body_mind::Npc;

pub enum ObjectType {

    Spike,
    MovingSpike(Direction),
    Portal(Destination),
    Brick,
    Body,

}

// texture is for 2d TUI rendering in the future 
pub struct Object {

    health: usize,
    invulnerable: bool,
    object_type: ObjectType,
    last_location: Location,
    location: Location,


}

impl Object {

    pub fn relocate(&mut self, location: Location){
        todo!();
    }

}


type TwoDimensionalPlain = Vec< Vec<Object> >;

pub struct Area { 

    plain: TwoDimensionalPlain,
    npcs: Vec<Npc>

}

impl Area {

    pub fn recieve_npc(npc: Npc, location: Location){
        todo!("cant insert npcs yet!");
    }

}