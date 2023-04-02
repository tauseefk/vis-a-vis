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
pub struct GridCoords<T> {
    pub x: T,
    pub y: T,
}

impl From<&GridCoords<i32>> for GridCoords<f32> {
    fn from(value: &GridCoords<i32>) -> Self {
        GridCoords {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<&GridCoords<f32>> for GridCoords<i32> {
    fn from(value: &GridCoords<f32>) -> Self {
        GridCoords {
            x: (value.x as i32).max(0),
            y: (value.y as i32).max(0),
        }
    }
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

pub enum Pivot {
    Center,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Pivot {
    pub fn coords(&self, tile_coords: &GridCoords<i32>) -> GridCoords<f32> {
        let tile_coords_f: GridCoords<f32> = tile_coords.into();
        match self {
            Pivot::Center => tile_coords_f,
            Pivot::TopRight => GridCoords {
                x: tile_coords_f.x + 0.5,
                y: tile_coords_f.y + 0.5,
            },
            Pivot::BottomRight => GridCoords {
                x: tile_coords_f.x + 0.5,
                y: tile_coords_f.y - 0.5,
            },
            Pivot::BottomLeft => GridCoords {
                x: tile_coords_f.x - 0.5,
                y: tile_coords_f.y - 0.5,
            },
            Pivot::TopLeft => GridCoords {
                x: tile_coords_f.x - 0.5,
                y: tile_coords_f.y + 0.5,
            },
        }
    }
}

pub struct Visibility<'test> {
    world: &'test World,
    is_omniscient: bool,
    max_visible_distance: i32,
    visible_tiles: HashSet<GridCoords<i32>>,
    observer: GridCoords<i32>,
}

impl<'test> Visibility<'test> {
    pub fn new(world: &'test World, is_omniscient: bool, max_visible_distance: i32) -> Self {
        Self {
            world,
            is_omniscient,
            max_visible_distance,
            visible_tiles: HashSet::new(),
            observer: GridCoords { x: 0, y: 0 },
        }
    }

    pub fn is_tile_visible(
        &self,
        observer_coords: &GridCoords<i32>,
        tile_coords: &GridCoords<i32>,
    ) -> bool {
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

        if !self.is_in_bounds(observer_coords) && !self.is_in_bounds(tile_coords) {
            panic!("Coordinate out of bounds!");
        }

        if self.world.tiles.len() == 1 || *observer_coords == *tile_coords || self.is_omniscient {
            true
        } else {
            false
        }
    }

    fn slope(&self, tile: &GridCoords<i32>, pivot: Pivot) -> f32 {
        return (tile.y - self.observer.y) as f32 / (tile.x - self.observer.y) as f32;
    }

    // assuming we're only concerned with the north - north - east octant
    // we're moving upwards so x = y * m;
    fn point_on_scan_line(&self, depth: i32, slope: f32) -> GridCoords<i32> {
        let y = depth;
        let x = y as f32 * slope;

        GridCoords { x: x as i32, y }
    }

    pub fn compute_visible_tiles(&self) -> HashSet<GridCoords<i32>> {
        HashSet::new()
    }

    fn compute_visible_tiles_in_octant(
        &self,
        // observer: &GridCoords, this doesn't change during each call
        current_depth: i32,
        min_slope: f32,
        max_slope: f32,
    ) {
    }

    fn grid_coord_to_idx(&self, tile_coords: &GridCoords<i32>) -> usize {
        if !self.is_in_bounds(tile_coords) {
            panic!("Tile not in bounds");
        }

        let w = self.world.width;

        (tile_coords.x * w + tile_coords.y) as usize
    }

    fn is_in_bounds(&self, tile_coords: &GridCoords<i32>) -> bool {
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

        let visibility = Visibility::new(&world, false, 3);

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

        let visibility = Visibility::new(&world, false, 3);

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

        let visibility = Visibility::new(&world, false, 3);

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

        let visibility = Visibility::new(&world, false, 3);

        let observer = GridCoords { x: 0, y: 0 };

        let tile = GridCoords { x: 2, y: 2 };

        let is_visible = visibility.is_tile_visible(&observer, &tile);
        assert!(is_visible);
    }
}
