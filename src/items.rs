//texture order:
//  none
//  bottom
//  none
//  none
//  top
//  none

pub const ITEMS: [Item; 7] = [
    Item {
        name: "Air",
        id: 0,
        is_transparent: true,
        is_solid: false,
        textures: [0, 0, 0, 0, 0, 0],
    },
    Item {
        name: "Dirt",
        id: 1,
        is_transparent: false,
        is_solid: true,
        textures: [1, 1, 1, 1, 1, 1],
    },
    Item {
        name: "Grass",
        id: 2,
        is_transparent: false,
        is_solid: true,
        textures: [2, 1, 2, 2, 7, 2],
    },
    Item {
        name: "Stone",
        id: 3,
        is_transparent: false,
        is_solid: true,
        textures: [0, 0, 0, 0, 0, 0],
    },
    Item {
        name: "Glass",
        id: 4,
        is_transparent: true,
        is_solid: true,
        textures: [3, 3, 3, 3, 3, 3],
    },
    Item {
        name: "Wood",
        id: 5,
        is_transparent: false,
        is_solid: true,
        textures: [5, 6, 5, 5, 6, 5],
    },
    Item {
        name: "Leave",
        id: 6,
        is_transparent: true,
        is_solid: true,
        textures: [16, 16, 16, 16, 16, 16],
    },
];

#[allow(dead_code)]
pub enum Items {
    Air,
    Dirt,
    Grass,
    Stone,
    Glass,
    Wood,
    Leave,
}

pub struct Item {
    pub name: &'static str,
    pub id: u16,
    pub is_transparent: bool,
    pub is_solid: bool,
    pub textures: [u16; 6],
}
