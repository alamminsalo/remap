use super::Tile;
use geo::{Coordinate, Rect};
use std::convert::{From, Into};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
}

impl Viewport {
    // returns osm tiles that intersect with this viewport
    // https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Mathematics
    pub fn tiles(&self, z: u8) -> Vec<Tile> {
        let a = Tile::from_lonlat(self.lon_min, self.lat_max, z);
        let b = Tile::from_lonlat(self.lon_max, self.lat_min, z).add(1);

        // return positions
        iproduct!(a.y..b.y, a.x..b.x)
            .map(|(y, x)| Tile { x, y, z })
            .collect()
    }
}

impl From<Rect<f64>> for Viewport {
    fn from(item: Rect<f64>) -> Self {
        Self {
            lon_min: item.min.x,
            lon_max: item.max.x,
            lat_min: item.min.y,
            lat_max: item.min.x,
        }
    }
}

impl Into<Rect<f64>> for Viewport {
    fn into(self) -> Rect<f64> {
        Rect {
            min: Coordinate {
                x: self.lon_min,
                y: self.lat_min,
            },
            max: Coordinate {
                x: self.lon_max,
                y: self.lat_max,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tiles_in_bb_1() {
        let tiles = Viewport {
            lon_min: 29.59,
            lon_max: 29.98,
            lat_min: 62.56,
            lat_max: 62.65,
        }
        .tiles(12);

        dbg!(&tiles);

        assert_eq!(tiles.len(), 18);
        assert_eq!(tiles[0].x, 2384);
        assert_eq!(tiles[0].y, 1126);
        assert_eq!(tiles[0].z, 12);
    }

    #[test]
    fn test_tiles_in_bb_2() {
        let tiles = Viewport {
            lon_min: -21.4326,
            lat_max: 54.74,
            lon_max: 80.0,
            lat_min: 24.88,
        }
        .tiles(4);

        dbg!(&tiles);

        assert_eq!(tiles.len(), 10);
        assert_eq!(tiles[0].x, 7);
        assert_eq!(tiles[0].y, 5);
        assert_eq!(tiles[1].x, 8);
        assert_eq!(tiles[1].y, 5);
        assert_eq!(tiles[2].x, 9);
        assert_eq!(tiles[2].y, 5);
        assert_eq!(tiles[3].x, 10);
        assert_eq!(tiles[3].y, 5);
        assert_eq!(tiles[4].x, 11);
        assert_eq!(tiles[4].y, 5);
        assert_eq!(tiles[5].x, 7);
        assert_eq!(tiles[5].y, 6);
        assert_eq!(tiles[6].x, 8);
        assert_eq!(tiles[6].y, 6);
        assert_eq!(tiles[7].x, 9);
        assert_eq!(tiles[7].y, 6);
        assert_eq!(tiles[8].x, 10);
        assert_eq!(tiles[8].y, 6);
        assert_eq!(tiles[9].x, 11);
        assert_eq!(tiles[9].y, 6);
    }

    #[test]
    fn test_tiles_in_bb_3() {
        let tiles = Viewport {
            lon_min: 29.54,
            lat_max: 62.65,
            lon_max: 30.18,
            lat_min: 62.52,
        }
        .tiles(11);

        dbg!(&tiles);

        assert_eq!(tiles.len(), 8);
        assert_eq!(tiles[0].x, 1192);
        assert_eq!(tiles[0].y, 563);
        assert_eq!(tiles[7].x, 1195);
        assert_eq!(tiles[7].y, 564);
    }

    #[test]
    fn test_tile_1() {
        let z = 8;
        let mut a = Tile { x: 100, y: 100, z };

        let zn = 2u32.pow(z as u32) as f64;
        let size = 360.0 / zn;

        for _ in 1..1000 {
            let lon_d = a.se().0 - a.nw().0;
            assert_eq!(size, lon_d);
            a = a.add(1);
        }
    }
}
