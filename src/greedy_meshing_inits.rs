use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

use crate::{chunk::Cube, items};

impl Voxel for Cube {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.id == 0 {
            VoxelVisibility::Empty
        } else if items::ITEMS[self.id as usize].is_transparent == true {
            VoxelVisibility::Translucent
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Cube {
    type MergeValue = u16;
    type MergeValueFacingNeighbour = u8;

    fn merge_value(&self) -> Self::MergeValue {
        self.id
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        self.light_level
    }
}