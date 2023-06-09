use std::{collections::HashSet, f32::INFINITY};

fn main() {
    #[rustfmt::skip]
        let tiles = vec![
            '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_',
            '_', '_', '_', '_', '_',
            '_', 'x', '_', '_', '_',
            '_', '_', '_', '_', '_'
        ];

    let tiles = tiles.iter().map(|value| value.into()).collect();
    let world = World {
        tiles,
        width: 5,
        height: 5,
    };

    let mut visibility = Visibility::new(&world, false, 4);

    let coords = vec![
        [
            GridCoords { x: 0, y: 4 },
            GridCoords { x: 1, y: 4 },
            GridCoords { x: 2, y: 4 },
            GridCoords { x: 3, y: 4 },
            GridCoords { x: 4, y: 4 },
        ],
        [
            GridCoords { x: 0, y: 3 },
            GridCoords { x: 1, y: 3 },
            GridCoords { x: 2, y: 3 },
            GridCoords { x: 3, y: 3 },
            GridCoords { x: 4, y: 3 },
        ],
        [
            GridCoords { x: 0, y: 2 },
            GridCoords { x: 1, y: 2 },
            GridCoords { x: 2, y: 2 },
            GridCoords { x: 3, y: 2 },
            GridCoords { x: 4, y: 2 },
        ],
        [
            GridCoords { x: 0, y: 1 },
            GridCoords { x: 1, y: 1 },
            GridCoords { x: 2, y: 1 },
            GridCoords { x: 3, y: 1 },
            GridCoords { x: 4, y: 1 },
        ],
        [
            GridCoords { x: 0, y: 0 },
            GridCoords { x: 1, y: 0 },
            GridCoords { x: 2, y: 0 },
            GridCoords { x: 3, y: 0 },
            GridCoords { x: 4, y: 0 },
        ],
    ];

    let slopes: Vec<Vec<f32>> = vec![
        vec![INFINITY, 4.00, 2.00, 1.33, 1.00],
        vec![INFINITY, 3.00, 1.50, 1.00, 0.75],
        vec![INFINITY, 2.00, 1.00, 0.67, 0.50],
        vec![INFINITY, 1.00, 0.50, 0.33, 0.25],
        vec![0.00, 0.00, 0.00, 0.00, 0.00],
    ];
    for i in 0..5 {
        for j in 0..5 {
            let coord = &coords[i][j];
            let slope = visibility.slope(&coord, crate::Pivot::Center);
            print!("{:.2}  ", slope);
        }
        println!("");
    }

    for i in 0..5 {
        for j in 0..5 {
            let slope = slopes[i][j];
            let coord = visibility.point_on_scan_line(4 - i as i32, slope);
            print!("({},{})  ", coord.x, coord.y);
        }
        println!("");
    }

    let vis_tiles = visibility.compute_visible_tiles();
    println!("{:?}", vis_tiles);
}

pub struct World {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

#[derive(Clone, PartialEq)]
pub enum TileType {
    Transparent,
    Opaque,
}

impl From<&char> for TileType {
    fn from(value: &char) -> Self {
        match value {
            'o' => TileType::Opaque,
            '_' => TileType::Transparent,
            _ => panic!("Encountered improbable tiletype"),
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

    fn get_tile_type(&self, tile_coords: &GridCoords<i32>) -> TileType {
        let idx = self.grid_coord_to_idx(tile_coords);
        self.world.tiles[idx].clone()
    }

    pub fn slope(&self, tile: &GridCoords<i32>, pivot: Pivot) -> f32 {
        let target = pivot.coords(tile);
        return (target.y - self.observer.y as f32) / (target.x - self.observer.x as f32);
    }

    /// 4\33333|22222/1
    /// 44\3333|2222/11
    /// 444\333|222/111
    /// 4444\33|22/1111
    /// 44444\3|2/11111
    /// ------@_@------
    /// 55555/6|7\88888
    /// 5555/66|77\8888
    /// 555/666|777\888
    /// 55/6666|7777\88
    /// 5/66666|77777\8
    /// assuming we're only concerned with the octant 1
    /// scan lines are vertical so y = mx
    fn point_on_scan_line(&self, depth: i32, slope: f32) -> GridCoords<i32> {
        if slope.is_infinite() {
            GridCoords {
                x: self.observer.x,
                y: (self.observer.y + depth).min(self.max_visible_distance),
            }
        } else {
            let x = (self.observer.x + depth).min(self.max_visible_distance);
            let y = (x as f32 * slope).min(self.max_visible_distance as f32);

            GridCoords { x, y: y as i32 }
        }
    }

    pub fn compute_visible_tiles(&mut self) -> HashSet<GridCoords<i32>> {
        self.compute_visible_tiles_in_octant(1, 0., 1.);
        self.visible_tiles.clone()
    }

    fn compute_visible_tiles_in_octant(
        &mut self,
        // observer: &GridCoords, this doesn't change during each call
        current_depth: i32,
        mut min_slope: f32,
        max_slope: f32,
    ) {
        if current_depth > self.max_visible_distance {
            return;
        }

        let mut is_first = true;
        let mut previous = self.point_on_scan_line(current_depth, min_slope);
        let mut current = self.point_on_scan_line(current_depth, min_slope);
        let end = self.point_on_scan_line(current_depth, max_slope);

        while current.y < end.y {
            println!("adding current to visible");
            self.visible_tiles.insert(current.clone());

            match (
                is_first,
                self.get_tile_type(&previous),
                self.get_tile_type(&current),
            ) {
                // first opaque cell after at least one transparent
                (false, TileType::Transparent, TileType::Opaque) => {
                    let next_max_slope = self.slope(&current, Pivot::BottomRight);
                    self.compute_visible_tiles_in_octant(
                        current_depth + 1,
                        min_slope,
                        next_max_slope,
                    );
                }
                // first transparent cell after at least one opaque
                (false, TileType::Opaque, TileType::Transparent) => {
                    min_slope = self.slope(&current, Pivot::TopLeft);
                }
                // do nothing
                (false, TileType::Transparent, TileType::Transparent) => {}
                (false, TileType::Opaque, TileType::Opaque) => {}
                (true, _, _) => {
                    is_first = false;
                }
            };
            previous = current.clone();
            current.y += 1;
        }
        // TODO: uncomment after encountering the edge case
        // see through last group of transparent cells in a row
        if self.get_tile_type(&previous) == TileType::Transparent {
            self.compute_visible_tiles_in_octant(current_depth + 1, min_slope, max_slope);
        }
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
    use std::f32::INFINITY;

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
    fn returns_all_visible_tiles() {
        #[rustfmt::skip]
        let tiles = vec![
            '_', '_', '_',
            '_', '_', '_',
            '_', '_', '_'
        ];

        let world = World {
            tiles: tiles.iter().map(|value| value.into()).collect(),
            width: 3,
            height: 3,
        };

        let mut visibility = Visibility::new(&world, false, 4);

        let visible_tiles = visibility.compute_visible_tiles();
        let visible_tiles: Vec<GridCoords<i32>> = visible_tiles.into_iter().collect();
        assert_eq!(visible_tiles, [])
    }

    // _  _  _
    // _  _  _
    // @  _  _
    // tile columns is reversed coz vec indices are ordered ascending
    #[test]
    fn get_slopes() {
        #[rustfmt::skip]
        let tiles = vec![
            '_', '_', '_', '_',
            '_', '_', '_', '_',
            '_', '_', '_', '_',
            '_', '_', '_', '_'
        ];

        let tiles = tiles.iter().map(|value| value.into()).collect();
        let world = World {
            tiles,
            width: 3,
            height: 3,
        };

        let visibility = Visibility::new(&world, false, 4);

        let coords = vec![
            [
                GridCoords { x: 0, y: 3 },
                GridCoords { x: 1, y: 3 },
                GridCoords { x: 2, y: 3 },
                GridCoords { x: 3, y: 3 },
            ],
            [
                GridCoords { x: 0, y: 2 },
                GridCoords { x: 1, y: 2 },
                GridCoords { x: 2, y: 2 },
                GridCoords { x: 3, y: 2 },
            ],
            [
                GridCoords { x: 0, y: 1 },
                GridCoords { x: 1, y: 1 },
                GridCoords { x: 2, y: 1 },
                GridCoords { x: 3, y: 1 },
            ],
            [
                GridCoords { x: 0, y: 0 },
                GridCoords { x: 1, y: 0 },
                GridCoords { x: 2, y: 0 },
                GridCoords { x: 3, y: 0 },
            ],
        ];

        let slopes = vec![
            vec![INFINITY, 3.00, 1.50, 1.00],
            vec![INFINITY, 2.00, 1.00, 0.67],
            vec![INFINITY, 1.00, 0.50, 0.33],
            vec![0.00, 0.00, 0.00, 0.00],
        ];

        for i in 0..4 {
            for j in 0..4 {
                let coord = &coords[i][j];
                let slope = visibility.slope(&coord, crate::Pivot::Center);
                if !slope.is_nan() && !slope.is_infinite() {
                    let abs_diff = (slope - slopes[i][j]).abs();
                    assert!(abs_diff < 0.01);
                }
            }
        }
    }
}
