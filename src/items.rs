//texture order:
//  none
//  bottom
//  none
//  none
//  top
//  none

pub const ITEMS: [Item; 5] = [
    Item {
        name: "Air",
        id: 0,
        is_opaque: false,
        is_solid: false,
        textures: [0, 0, 0, 0, 0, 0],
    },
    Item {
        name: "Dirt",
        id: 1,
        is_opaque: true,
        is_solid: true,
        textures: [1, 1, 1, 1, 1, 1],
    },
    Item {
        name: "Grass",
        id: 2,
        is_opaque: true,
        is_solid: true,
        textures: [2, 1, 2, 2, 7, 2],
    },
    Item {
        name: "Stone",
        id: 3,
        is_opaque: true,
        is_solid: true,
        textures: [0, 0, 0, 0, 0, 0],
    },
    Item {
        name: "Glass",
        id: 4,
        is_opaque: false,
        is_solid: true,
        textures: [3, 3, 3, 3, 3, 3],
    },
];

#[allow(dead_code)]
pub enum Items {
    Air,
    Dirt,
    Grass,
    Stone,
    Glass,
}

pub struct Item {
    pub name: &'static str,
    pub id: u16,
    pub is_opaque: bool,
    pub is_solid: bool,
    pub textures: [u16; 6],
}
