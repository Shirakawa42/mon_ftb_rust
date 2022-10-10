pub const FACES: [[i8; 3]; 6] = [
    [-1, 0, 0], // left
    [0, -1, 0], // bottom
    [0, 0, -1], //
    [1, 0, 0],  // right
    [0, 1, 0],  // top
    [0, 0, 1],  // 
];

pub const ITEMS: [Item; 8] = [
    Item {
        name: "Air",
        id: 0,
        is_transparent: true,
        is_solid: false,
        textures: [0, 0, 0, 0, 0, 0],
        light_multiplier: 1.0,
    },
    Item {
        name: "Dirt",
        id: 1,
        is_transparent: false,
        is_solid: true,
        textures: [1, 1, 1, 1, 1, 1],
        light_multiplier: 0.0,
    },
    Item {
        name: "Grass",
        id: 2,
        is_transparent: false,
        is_solid: true,
        textures: [2, 1, 2, 2, 7, 2],
        light_multiplier: 0.0,
    },
    Item {
        name: "Stone",
        id: 3,
        is_transparent: false,
        is_solid: true,
        textures: [0, 0, 0, 0, 0, 0],
        light_multiplier: 0.0,
    },
    Item {
        name: "Glass",
        id: 4,
        is_transparent: true,
        is_solid: true,
        textures: [3, 3, 3, 3, 3, 3],
        light_multiplier: 0.9,
    },
    Item {
        name: "Wood",
        id: 5,
        is_transparent: false,
        is_solid: true,
        textures: [5, 6, 5, 5, 6, 5],
        light_multiplier: 0.0,
    },
    Item {
        name: "Leave",
        id: 6,
        is_transparent: true,
        is_solid: true,
        textures: [16, 16, 16, 16, 16, 16],
        light_multiplier: 0.8,
    },
    Item {
        name: "Sand",
        id: 7,
        is_transparent: false,
        is_solid: true,
        textures: [10, 10, 10, 10, 10, 10],
        light_multiplier: 0.0,
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
    Sand,
}

pub struct Item {
    pub name: &'static str,
    pub id: u16,
    pub is_transparent: bool,
    pub is_solid: bool,
    pub textures: [u16; 6],
    pub light_multiplier: f32,
}
