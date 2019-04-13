use googleprojection as wgs84;

#[derive(Clone, Default, PartialEq, Copy)]
pub struct LonLat {
    pub lon: f64,
    pub lat: f64,
}

impl LonLat {
    pub fn px(&self, z: usize) -> Px {
        let (x, y) = wgs84::from_ll_to_pixel(&(self.lon, self.lat), z).unwrap_or((0.0, 0.0));
        Px {
            x: x as i64,
            y: y as i64,
        }
    }

    pub fn translate(&self, lon: f64, lat: f64) -> Self {
        Self {
            lon: self.lon + lon,
            lat: self.lat + lat,
        }
    }
}

impl Into<(f64, f64)> for LonLat {
    fn into(self) -> (f64, f64) {
        (self.lon, self.lat)
    }
}

impl From<(f64, f64)> for LonLat {
    fn from(ll: (f64, f64)) -> Self {
        Self {
            lon: ll.0,
            lat: ll.1,
        }
    }
}

#[derive(Clone, Default, PartialEq, Copy)]
pub struct Px {
    pub x: i64,
    pub y: i64,
}

impl Px {
    pub fn lonlat(&self, z: usize) -> LonLat {
        let (lon, lat) =
            wgs84::from_pixel_to_ll(&(self.x as f64, self.y as f64), z).unwrap_or((0.0, 0.0));
        LonLat { lon, lat }
    }

    pub fn translate(&self, x: i64, y: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Into<(i64, i64)> for Px {
    fn into(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl From<(i64, i64)> for Px {
    fn from(xy: (i64, i64)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl From<(f64, f64)> for Px {
    fn from(xy: (f64, f64)) -> Self {
        Self {
            x: xy.0 as i64,
            y: xy.1 as i64,
        }
    }
}
