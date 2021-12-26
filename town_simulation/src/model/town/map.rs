use crate::model::building::BuildingId;
use crate::model::town::map::TownBlock::*;
use crate::model::town::map::TownLot::*;
use derive_getters::Getters;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TownLot {
    EmptyLot,
    BuildingLot(BuildingId),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TownBlock {
    EmptyBlock,
    SmallBuildings([TownLot; 4]),
}

impl TownBlock {
    pub fn empty() -> Self {
        SmallBuildings([EmptyLot; 4])
    }
}

#[derive(Getters, Clone, Debug, PartialEq)]
pub struct TownMap {
    width: usize,
    height: usize,
    blocks: Vec<TownBlock>,
}

impl TownMap {
    pub fn new(width: usize, height: usize, blocks: Vec<TownBlock>) -> Self {
        if width * height != blocks.len() {
            panic!(
                "Width {} & height {} don't match {} blocks!",
                width,
                height,
                blocks.len()
            );
        }

        Self {
            width,
            height,
            blocks,
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            blocks: vec![EmptyBlock; width * height],
        }
    }

    pub fn is_lot_free(&self, block: usize, lot: usize) -> bool {
        if let Some(SmallBuildings(buildings)) = self.blocks.get(block) {
            return buildings[lot] == EmptyLot;
        }

        true
    }

    pub fn add_building(&mut self, id: BuildingId, block: usize, lot: usize) {
        if self.blocks[block] == EmptyBlock {
            self.blocks[block] = TownBlock::empty();
        }

        if self.is_lot_free(block, lot) {
            if let Some(SmallBuildings(buildings)) = self.blocks.get_mut(block) {
                buildings[lot] = BuildingLot(id);
                return;
            }
        }

        panic!(
            "Failed to add building {} to lot {} of block {}!",
            id.id(),
            lot,
            block
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_lot_free() {
        let id = BuildingId::new(0);
        let block = SmallBuildings([EmptyLot, BuildingLot(id), EmptyLot, EmptyLot]);
        let map = TownMap::new(2, 1, vec![EmptyBlock, block]);

        assert!(map.is_lot_free(0, 0));
        assert!(map.is_lot_free(0, 1));
        assert!(map.is_lot_free(0, 2));
        assert!(map.is_lot_free(0, 3));

        assert!(map.is_lot_free(1, 0));
        assert!(!map.is_lot_free(1, 1));
        assert!(map.is_lot_free(1, 2));
        assert!(map.is_lot_free(1, 3));
    }
}
