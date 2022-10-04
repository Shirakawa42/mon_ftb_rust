use block_mesh::{Voxel, VoxelVisibility, MergeVoxel};

use crate::{chunk::Cube, items};

impl Voxel for Cube {
    fn get_visibility(&self) -> VoxelVisibility {
        if items::ITEMS[self.id as usize].is_opaque == false {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Cube {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}