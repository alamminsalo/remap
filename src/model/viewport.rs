use super::Tile;
use geo::{Coordinate, Rect};
use std::convert::{From, Into};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
    // zoomlevel
    pub z: u8,
}

impl Viewport {
    // returns osm tiles that intersect with this viewport
    // https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Mathematics
    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        let z = self.z;
        let a = Tile::from_lonlat(self.lon_min, self.lat_max, z);
        let b = Tile::from_lonlat(self.lon_max, self.lat_min, z).add(1);

        // return positions
        iproduct!(a.y..b.y, a.x..b.x).map(move |(y, x)| Tile { x, y, z })
    }

    // calculates pixel offset for tile
    pub fn pixel_offset(&self, tile: &Tile) -> (i32, i32) {
        // let res = 156543.03 * self.lat_max.cos() / 2f64.powi(self.z as i32);
        // dbg!(&res);
        // let nw = (self.lon_min, self.lat_max);
        // dbg!(&nw);
        // let t_nw = tile.nw();
        // let d_lon = (t_nw.0 - nw.0) * res;
        // let d_lat = (t_nw.1 - nw.1) * res;
        // dbg!(d_lon);
        // dbg!(d_lat);
        // (d_lon as i32, d_lat as i32)

        // just use linear interpolation
        let nw = tile.nw();
        let se = tile.se();
        let f_lon = (self.lon_min - nw.0) / (se.0 - nw.0);
        let f_lat = (self.lat_max - nw.1) / (se.1 - nw.1);
        ((f_lon * -256.0) as i32, (f_lat * -256.0) as i32)
    }
}

impl From<Rect<f64>> for Viewport {
    fn from(item: Rect<f64>) -> Self {
        Self {
            lon_min: item.min.x,
            lon_max: item.max.x,
            lat_min: item.min.y,
            lat_max: item.min.x,
            z: 0,
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
