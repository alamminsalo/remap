use super::{LonLat, Px, Viewport};
use std::f64::consts::PI;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: usize,
}

impl Tile {
    pub fn from_lonlat(lon: f64, lat: f64, z: usize) -> Self {
        let znx = 2f64.powi(z as i32);
        let zny = 2f64.powi(z as i32 - 1);
        let lat_rad = lat.to_radians();
        Self {
            x: (znx * ((lon + 180.0) / 360.0)) as u32,
            y: ((1.0 - (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() / PI) * zny) as u32,
            z,
        }
    }

    pub fn add(&self, n: i32) -> Tile {
        Tile {
            x: (self.x as i32 + n) as u32,
            y: (self.y as i32 + n) as u32,
            z: self.z,
        }
    }
    // returns northwest point of this tile
    pub fn nw(&self) -> (f64, f64) {
        let nz = 2f64.powi(self.z as i32);
        let lon = (self.x as f64 / nz) * 360.0 - 180.0;
        let lat = (PI * (1.0 - 2.0 * self.y as f64 / nz))
            .sinh()
            .atan()
            .to_degrees();
        (lon, lat)
    }
    // returns southeast point of this tile
    pub fn se(&self) -> (f64, f64) {
        self.add(1).nw()
    }
    // returns bounding box for this tile (mercator)
    // https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Mathematics
    pub fn bbox(&self) -> Viewport {
        let (lon_min, lat_max) = self.nw();
        let (lon_max, lat_min) = self.se();

        Viewport {
            lon_min,
            lon_max,
            lat_min,
            lat_max,
            z: self.z,
        }
    }

    pub fn pixels(&self) -> Px {
        let ll: LonLat = self.nw().into();
        ll.px(self.z)
    }
}

/// Raster tile layer
#[derive(Default, PartialEq, Clone)]
pub struct TileLayer {
    /// Tile url
    pub url: String,
    /// Tile url suffix eg. '.png?apikey=foobar'
    pub suffix: String,
    /// Controls whether layer is visible on map
    pub visible: bool,
}

impl TileLayer {
    pub fn new(url: &str, suffix: &str) -> Self {
        Self {
            url: url.into(),
            suffix: suffix.into(),
            visible: true,
        }
    }
    pub fn tile_url(&self, tile: &Tile) -> String {
        format!(
            "{url}/{z}/{x}/{y}{suffix}",
            url = &self.url,
            z = tile.z,
            x = tile.x,
            y = tile.y,
            suffix = &self.suffix,
        )
    }
}
