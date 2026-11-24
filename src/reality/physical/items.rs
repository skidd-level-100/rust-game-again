
pub enum ItemType {

    Bow,
    Sword,
    Armour,
    Brick,

}

pub struct Item {

    r#type: ItemType,
    count: usize,
    consumable: bool,

}