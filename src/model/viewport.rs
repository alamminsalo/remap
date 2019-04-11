use super::Tile;
use geo::{Coordinate, Rect};
use googleprojection as wgs84;
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
    // Creates new viewport from center coordinate, pixel bounds and zoom level
    pub fn new(center: &Coordinate<f64>, size: (i32, i32), zoom: u8) -> Self {
        let (x, y) = ((size.0 / 2) as f64, (size.1 / 2) as f64);
        let (c_x, c_y) =
            wgs84::from_ll_to_pixel(&(center.x, center.y), zoom as usize).unwrap_or((0.0, 0.0));
        let nw: (f64, f64) =
            wgs84::from_pixel_to_ll(&(c_x - x, c_y - y), zoom as usize).unwrap_or((0.0, 0.0));
        let se: (f64, f64) =
            wgs84::from_pixel_to_ll(&(c_x + x, c_y + y), zoom as usize).unwrap_or((0.0, 0.0));

        // console!(log, "ce", &c_x, &c_y);
        // console!(log, "nw", &nw.0, &nw.1);
        // console!(log, "se", &se.0, &se.1);

        Self {
            lon_min: nw.0,
            lon_max: se.0,
            lat_max: nw.1,
            lat_min: se.1,
            z: zoom,
        }
    }

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
    pub fn pixel_offset(&self, tile: &Tile) -> (i64, i64) {
        // just use linear interpolation
        // let nw = tile.nw();
        // let se = tile.se();
        // let f_lon = (self.lon_min - nw.0) / (se.0 - nw.0);
        // let f_lat = (self.lat_max - nw.1) / (se.1 - nw.1);
        // ((f_lon * -256.0) as i32, (f_lat * -256.0) as i32)

        // use pixel coordinates diff
        let (x, y) = self.pixels();
        let (t_x, t_y) = tile.pixels();
        (t_x - x, t_y - y)
    }

    // get pixel coordinates
    pub fn pixels(&self) -> (i64, i64) {
        let px: (f64, f64) =
            wgs84::from_ll_to_pixel(&(self.lon_min, self.lat_max), self.z as usize)
                .unwrap_or((0.0, 0.0));
        (px.0 as i64, px.1 as i64)
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
