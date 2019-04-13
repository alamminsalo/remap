pub mod position;
pub mod tile;
pub mod viewport;

pub use tile::Tile;
pub use viewport::Viewport;

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
            z: 12,
        }
        .tiles()
        .collect::<Vec<Tile>>();

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
            z: 4,
        }
        .tiles()
        .collect::<Vec<Tile>>();

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
            z: 11,
        }
        .tiles()
        .collect::<Vec<Tile>>();

        dbg!(&tiles);

        assert_eq!(tiles.len(), 8);
        assert_eq!(tiles[0].x, 1192);
        assert_eq!(tiles[0].y, 563);
        assert_eq!(tiles[7].x, 1195);
        assert_eq!(tiles[7].y, 564);
    }

    #[test]
    fn test_tile_1() {
        let a = Tile {
            x: 100,
            y: 100,
            z: 8,
        };
        let (lon, lat) = a.nw();

        assert_eq!(lon, -39.375);
        assert!((lat - 36.5978891330702) < 1e-10);
    }

    #[test]
    fn test_tile_2() {
        let a = Tile {
            x: 999,
            y: 999,
            z: 12,
        };
        let (lon, lat) = a.nw();

        assert_eq!(lon, -92.197265625);
        assert!((lat - 67.37369797436554) < 1e-10);
    }

    // #[test]
    // fn test_pixel_offset_1() {
    //     let vw = Viewport {
    //         lon_min: 29.71,
    //         lon_max: 29.87539,
    //         lat_min: 62.557,
    //         lat_max: 62.62997,
    //         z: 12,
    //     };

    //     let tile = vw.tiles().next().unwrap();
    //     dbg!(&tile);
    //     let offset = vw.pixel_offset(&tile);
    //     dbg!(&offset);

    //     assert_eq!(offset.0, 0);
    // }

    // #[test]
    // fn test_pixel_offset_2() {
    //     let vw = Viewport {
    //         lon_min: 29.685,
    //         lon_max: 29.87539,
    //         lat_min: 62.557,
    //         lat_max: 62.638,
    //         z: 12,
    //     };

    //     let tile = vw.tiles().next().unwrap();
    //     dbg!(&tile);
    //     let offset = vw.pixel_offset(&tile);
    //     dbg!(&offset);

    //     assert_eq!(offset.0, 0);
    // }
}
