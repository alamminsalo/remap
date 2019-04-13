use super::position::{LonLat, Px};
use super::Tile;
use googleprojection as wgs84;

// Viewport: boundingbox with zoom level
// and useful transformation logic
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub lon_min: f64,
    pub lon_max: f64,
    pub lat_min: f64,
    pub lat_max: f64,
    // zoomlevel
    pub z: usize,
}

impl Viewport {
    // Creates new viewport from center coordinate, pixel bounds and zoom level
    pub fn new(center: &LonLat, size_px: (i32, i32), zoom: usize) -> Self {
        let (dx, dy) = (size_px.0 / 2, size_px.1 / 2);
        let px = center.px(zoom);
        let nw = px.translate(-dx as i64, -dy as i64).lonlat(zoom);
        let se = px.translate(dx as i64, dy as i64).lonlat(zoom);

        Self {
            lon_min: nw.lon,
            lon_max: se.lon,
            lat_max: nw.lat,
            lat_min: se.lat,
            z: zoom,
        }
    }

    // translates by given pixel amount
    pub fn translate(&self, xy: (i32, i32)) -> Viewport {
        // original pixels
        let (vw_x, vw_y) = self.pixels();

        // translated pixels
        let (gx, gy) = (vw_x + xy.0 as i64, vw_y + xy.1 as i64);

        // new nw lonlat
        let nw =
            wgs84::from_pixel_to_ll(&(gx as f64, gy as f64), self.z as usize).unwrap_or((0.0, 0.0));

        // diff along axises
        let lon_d = self.lon_min - nw.0;
        let lat_d = self.lat_max - nw.1;

        Viewport {
            lon_min: self.lon_min + lon_d,
            lon_max: self.lon_max + lon_d,
            lat_min: self.lat_min + lat_d,
            lat_max: self.lat_max + lat_d,
            z: self.z,
        }
    }

    // naive center, use this in emergencies only
    fn naive_center(&self) -> (f64, f64) {
        (
            self.lon_min / 2.0 + self.lon_max / 2.0,
            self.lat_min / 2.0 + self.lat_max / 2.0,
        )
    }

    // Returns center of this viewport
    pub fn center(&self) -> LonLat {
        // nw pixel corner
        let (x0, y0) = self.pixels();

        // se pixel corner
        let (x1, y1): (f64, f64) =
            wgs84::from_ll_to_pixel(&(self.lon_max, self.lat_min), self.z as usize)
                .unwrap_or((x0 as f64, y0 as f64));

        // getn center lonlat from center world pixels
        let center_px: Px = ((x0 as f64) / 2.0 + x1 / 2.0, (y0 as f64) / 2.0 + y1 / 2.0).into();

        center_px.lonlat(self.z)
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
        let (x, y) = self.pixels();
        let (t_x, t_y) = tile.pixels();
        (t_x - x, t_y - y)
    }

    // get pixel coordinates (NW corner)
    pub fn pixels(&self) -> (i64, i64) {
        let px: (f64, f64) =
            wgs84::from_ll_to_pixel(&(self.lon_min, self.lat_max), self.z as usize)
                .unwrap_or((0.0, 0.0));
        (px.0 as i64, px.1 as i64)
    }
}
