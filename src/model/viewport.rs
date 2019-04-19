use super::position::{LonLat, Px};
use super::Tile;

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
        let nw = px.translate(&(-dx as i64, -dy as i64).into()).lonlat(zoom);
        let se = px.translate(&(dx as i64, dy as i64).into()).lonlat(zoom);

        Self {
            lon_min: nw.lon,
            lon_max: se.lon,
            lat_max: nw.lat,
            lat_min: se.lat,
            z: zoom,
        }
    }

    /// Translates by given pixel amount
    pub fn translate(&self, xy: &Px) -> Viewport {
        // translated pixels
        let px: Px = self.pixels().translate(&xy);

        // new nw lonlat
        let nw = px.lonlat(self.z);

        // diff along axises
        let lon_d = self.lon_min - nw.lon;
        let lat_d = self.lat_max - nw.lat;

        Viewport {
            lon_min: self.lon_min + lon_d,
            lon_max: self.lon_max + lon_d,
            lat_min: self.lat_min + lat_d,
            lat_max: self.lat_max + lat_d,
            z: self.z,
        }
    }

    /// Returns center of this viewport
    pub fn center(&self) -> LonLat {
        // nw pixel corner
        let nw_px = self.pixels();
        // se corner
        let se: LonLat = (self.lon_max, self.lat_min).into();
        // get center px and return lonlat from it
        nw_px.avg(&se.px(self.z)).lonlat(self.z)
    }

    /// Returns osm tiles that intersect with this viewport
    /// https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Mathematics
    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        let z = self.z;
        let a = Tile::from_lonlat(self.lon_min, self.lat_max, z);
        let b = Tile::from_lonlat(self.lon_max, self.lat_min, z).add(1);

        // return positions
        iproduct!(a.y..b.y, a.x..b.x).map(move |(y, x)| Tile { x, y, z })
    }

    /// Calculates pixel offset for tile
    pub fn pixel_offset(&self, tile: &Tile) -> Px {
        tile.pixels().distance(&self.pixels())
    }

    /// Get pixel coordinates (NW corner)
    pub fn pixels(&self) -> Px {
        let ll: LonLat = (self.lon_min, self.lat_max).into();
        ll.px(self.z)
    }

    /// Returns pixel coordinates for both nw, se borders
    pub fn pixel_bounds(&self) -> (Px, Px) {
        let nw: LonLat = (self.lon_min, self.lat_max).into();
        let se: LonLat = (self.lon_max, self.lat_min).into();
        (nw.px(self.z), se.px(self.z))
    }

    /// Creates viewport from pixel bounds and zoom level
    pub fn from_pixel_bounds(nw: Px, se: Px, z: usize) -> Self {
        let nw_ll = nw.lonlat(z);
        let se_ll = se.lonlat(z);

        Self {
            lon_min: nw_ll.lon,
            lat_max: nw_ll.lat,
            lon_max: se_ll.lon,
            lat_min: se_ll.lat,
            z: z,
        }
    }

    pub fn resize_keep_min_bounds(&self, offset: Px) -> Self {
        // resize outer viewport accordingly
        let (mut nw, mut se) = self.pixel_bounds();
        nw.x = (nw.x + offset.x).min(nw.x);
        nw.y = (nw.y + offset.y).min(nw.y);
        se.x = (se.x + offset.x).max(se.x);
        se.y = (se.y + offset.y).max(se.y);
        Viewport::from_pixel_bounds(nw, se, self.z)
    }
}
