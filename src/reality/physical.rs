// the solid world, blocks and bodies

pub mod items;
pub mod space;

use crate::reality::physical::space::{Direction, Destination, Location};
use crate::reality::physical::items::{Item, Inventory};
use crate::reality::body_mind::Npc;
use std::collections::HashMap;

pub type MultipleObjects = Vec<Object>;

pub enum ObjectType {

    MovingSpike(Direction),
    Portal(Destination), // Direction is what area number to move to.
    Container(Inventory),
    Item,
    Spike,
    Brick,
    Door,
    Body,

}

// texture is for 2d TUI rendering in the future 
pub struct Object {

    health: isize,
    base_health: isize,
    team: isize,
    count: isize,
    invulnerable: bool,
    blocks_own_team: bool,
    r#type: ObjectType,

}

impl Object {


    // creates objects which dont need specification such as a Brick,Door,Body or Spike
    pub fn new_boring(object: ObjectType) -> Object{

        match object {
            ObjectType::Spike => {
                let base_hp = 5;

                return Object {
                    health: base_hp,
                    base_health: base_hp,
                    team: 0,
                    count: 1,
                    invulnerable: false,
                    blocks_own_team: true,
                    r#type: ObjectType::Spike,
                }

            },
            ObjectType::MovingSpike(_) => {
                panic!("please use the Object::new_moving_spike() function instead");
            },
            ObjectType::Portal(_) => {
                panic!("please use the Object::new_portal() function instead, and make sure it has a valid destination!");
            },
            ObjectType::Brick => {
                let base_hp = 10;

                return Object {
                    health: base_hp,
                    base_health: base_hp,
                    team: 0,
                    count: 1,
                    invulnerable: false,
                    blocks_own_team: true,
                    r#type: ObjectType::Brick,
                }
            },
            ObjectType::Door => {
                let base_hp = 5;

                return Object {
                    health: base_hp,
                    base_health: base_hp,
                    team: 0,
                    count: 1,
                    invulnerable: false,
                    blocks_own_team: false,
                    r#type: ObjectType::Door,
                }
            },
            ObjectType::Body => {
                let base_hp = 20;
                
                return Object {
                    health: base_hp,
                    base_health: base_hp,
                    team: 1,
                    count: 1,
                    invulnerable: false,
                    blocks_own_team: true,
                    r#type: ObjectType::Body,
                }
            },
            ObjectType::Item => {
                panic!("please use Object::new_item function instead!");
            },
            ObjectType::Container(_) => {
                panic!("please use Object::new_container function instead");
            }
            _ => todo!("object creation not yet finished")

        }

    }

    pub fn new_moving_spike(direction: Direction) -> Object {

        let base_hp = 5;
        return Object {
            health: base_hp,
            base_health: base_hp,
            team: 0,
            count: 1,
            invulnerable: false,
            blocks_own_team: true,
            r#type: ObjectType::MovingSpike(direction),
        }

    }

    pub fn new_item(item: Item) -> Object {
        todo!("Item object creation not implemented yet");
    }

    pub fn new_portal(dest: Destination) -> Object{
        todo!("portals not implemented yet!");
    }

    pub fn new_container(inv: Inventory) -> Object {
        todo!("Containers not implemented yet!")
    }
}