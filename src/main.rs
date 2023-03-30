use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

pub struct World {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq)]
pub struct GridCoords {
    pub x: i32,
    pub y: i32,
}

pub enum TileType {
    Transparent,
    Opaque,
}

impl From<&char> for TileType {
    fn from(value: &char) -> Self {
        match value {
            'o' => TileType::Opaque,
            '_' => TileType::Transparent,
            _ => panic!("Encountered improbably tiletype"),
        }
    }
}

pub struct Visibility<'test> {
    world: &'test World,
    is_omniscient: bool,
    max_visible_distance: i32,
}

impl<'test> Visibility<'test> {
    pub fn is_tile_visible(&self, observer_coords: &GridCoords, tile_coords: &GridCoords) -> bool {
        // TODO: this should prob happen at the world construction
        if self.world.tiles.len() < 1 {
            panic!("World is too small.");
        }

        if self.world.tiles.len() != (self.world.width * self.world.height) as usize {
            panic!("World size is inconsistent");
        }

        if self.max_visible_distance < 1 {
            panic!("Can't see shit!");
        }

        if self.world.tiles.len() == 1 || *observer_coords == *tile_coords || self.is_omniscient {
            true
        } else {
            false
        }
    }

    pub fn compute_visible_tiles(&self, _observer_coords: &GridCoords) -> HashSet<GridCoords> {
        HashSet::new()
    }

    fn grid_coord_to_idx(&self, tile_coords: &GridCoords) -> usize {
        if !self.is_in_bounds(tile_coords) {
            panic!("Tile not in bounds");
        }

        let w = self.world.width;

        (tile_coords.x * w + tile_coords.y) as usize
    }

    fn is_in_bounds(&self, tile_coords: &GridCoords) -> bool {
        let x = tile_coords.x;
        let y = tile_coords.y;

        x >= 0 && y >= 0 && x < self.world.width && y < self.world.height
    }
}

#[cfg(test)]
mod tests {
    use crate::{GridCoords, Visibility, World};

    #[test]
    #[should_panic]
    fn panics_world_is_too_small() {
        let world = World {
            tiles: vec![],
            width: 0,
            height: 0,
        };

        let visibility = Visibility {
            world: &world,
            max_visible_distance: 3,
            is_omniscient: false,
        };

        let observer = GridCoords { x: 0, y: 0 };

        let tile = GridCoords { x: 0, y: 0 };

        let is_visible = visibility.is_tile_visible(&observer, &tile);
        assert!(is_visible);
    }

    #[test]
    fn returns_true_for_one_sized_world() {
        let tiles: Vec<char> = vec!['_'];
        let world = World {
            tiles: tiles.iter().map(|value| value.into()).collect(),
            width: 1,
            height: 1,
        };

        let visibility = Visibility {
            world: &world,
            max_visible_distance: 3,
            is_omniscient: false,
        };

        let observer = GridCoords { x: 0, y: 0 };

        let tile = GridCoords { x: 0, y: 0 };

        let is_visible = visibility.is_tile_visible(&observer, &tile);
        assert!(is_visible);
    }

    // @/x _
    //  _  _
    #[test]
    fn returns_true_for_itself() {
        #[rustfmt::skip]
        let tiles = vec![
            '_', '_',
            '_', '_'
        ];

        let world = World {
            tiles: tiles.iter().map(|value| value.into()).collect(),
            width: 2,
            height: 2,
        };

        let visibility = Visibility {
            world: &world,
            max_visible_distance: 3,
            is_omniscient: false,
        };

        let observer = GridCoords { x: 0, y: 0 };

        let tile = GridCoords { x: 0, y: 0 };

        let is_visible = visibility.is_tile_visible(&observer, &tile);
        assert!(is_visible);
    }

    // @  _  _
    // _  o  _
    // _  _  x
    #[test]
    fn returns_false_for_hidden_tile() {
        #[rustfmt::skip]
        let tiles = vec![
            '_', '_', '_',
            '_', 'o', '_',
            '_', '_', '_'
        ];

        let world = World {
            tiles: tiles.iter().map(|value| value.into()).collect(),
            width: 3,
            height: 1,
        };

        let visibility = Visibility {
            world: &world,
            max_visible_distance: 3,
            is_omniscient: false,
        };

        let observer = GridCoords { x: 0, y: 0 };

        let tile = GridCoords { x: 2, y: 2 };

        let is_visible = visibility.is_tile_visible(&observer, &tile);
        assert!(is_visible);
    }
}
