// BEHOLD REALITY ITSELF

mod mental;
mod physical;
mod body_mind;

use crate::reality::body_mind::Npc; 

use crate::reality::physical::Area;

use std::collections::HashMap;

pub struct World {

    map: HashMap<String, Area>, // area name, the Area itself

}

