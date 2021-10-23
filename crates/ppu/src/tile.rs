use std::convert::From;

pub struct DataVec(Vec<bool>);

pub struct Tile([u8; 16]);

pub type Pixels = Vec<Vec<u8>>;

impl DataVec {
    pub fn new(data: u8) -> Self {
        let mut datavec = Vec::new();
        for i in 0..=7 {
            datavec.insert(0, data & (1 << i) != 0);
        }
        DataVec(datavec)
    }
}

impl From<Tile> for Pixels {
    fn from(tile: Tile) -> Self {
        let Tile(inner) = tile;
        inner
            .chunks(2)
            .map(|bytes| {
                let mut line = Vec::new();
                let DataVec(lower) = DataVec::new(bytes[0]);
                let DataVec(higher) = DataVec::new(bytes[1]);
                let iterator = lower.iter().zip(higher.iter()).enumerate();

                for (index, (lower, higher)) in iterator {
                    let higher = (*higher) as u8;
                    let lower = (*lower) as u8;
                    line.insert(index, (higher << 1) + lower);
                }
                line
            })
            .collect()
    }
}

#[cfg(test)]
mod test_tiles {
    use super::*;

    #[test]
    fn test_from_tile_to_map() {
        let tile = Tile([
            0x7C, 0x7C, 0x00, 0xC6, 0xC6, 0x00, 0x00, 0xFE, 0xC6, 0xC6, 0x00, 0xC6, 0xC6, 0x00,
            0x00, 0x00,
        ]);
        let map = Pixels::from(tile);
        let line1 = vec![0, 3, 3, 3, 3, 3, 0, 0];
        let line2 = vec![2, 2, 0, 0, 0, 2, 2, 0];
        let line3 = vec![1, 1, 0, 0, 0, 1, 1, 0];
        let line4 = vec![2, 2, 2, 2, 2, 2, 2, 0];
        let line5 = vec![3, 3, 0, 0, 0, 3, 3, 0];
        let line6 = vec![2, 2, 0, 0, 0, 2, 2, 0];
        let line7 = vec![1, 1, 0, 0, 0, 1, 1, 0];
        let line8 = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let dest = vec![line1, line2, line3, line4, line5, line6, line7, line8];
        assert_eq!(map, dest);
    }
}
