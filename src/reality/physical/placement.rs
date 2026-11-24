// thing that have to do with moving

pub struct Location {
    x: usize,
    y: usize
}

pub enum Direction {

    PlusX,
    PlusY,
    NegY,
    NegX,
    Still

}

pub enum Destination {

    AreaName(String)

}