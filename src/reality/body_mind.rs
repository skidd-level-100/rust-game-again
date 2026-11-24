use std::collections::HashMap;

use crate::reality::World;
use crate::reality::physical::Object;
use crate::reality::physical::placement::Direction;
use crate::reality::physical::items::{Item, ItemType};

use crate::reality::mental::Brain;

pub type Inventory = HashMap<ItemType, Item>;

pub struct Npc {

    pub texture: char, // renders this emoji in 2d rendering
    brain: Brain,
    body: Object,
    inventory: Inventory

}

pub enum Action {

    Attack(Direction),
    Place(Direction),
    Step(Direction),
    UseItem(Direction, Item)

}