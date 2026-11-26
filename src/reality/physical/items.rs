use std::collections::HashMap;

pub type Inventory = HashMap<ItemType, Item>;

pub enum ItemType {

    Bow,
    Sword,
    Armour,
    Brick,

}

pub struct Item {

    r#type: ItemType,
    count: isize,
    consumable: bool,

}