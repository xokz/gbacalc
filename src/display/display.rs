use crate::VRamManager;

use agb::{
    display::tiled::{MapLoan, RegularMap, TileSet, TileSetting},
    fixnum::Vector2D,
};

pub trait Drawing {
    fn print(
        &mut self,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
        text: &[u8],
        pos: &Vector2D<u16>,
    );
    fn fill(&mut self, vram: &mut VRamManager, tileset: &TileSet<'_>, tile_index: u16);
}

impl Drawing for MapLoan<'_, RegularMap> {
    fn print(
        &mut self,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
        text: &[u8],
        pos: &Vector2D<u16>,
    ) {
        let bytes = text;
        let mut pos = *pos;
        for byte in bytes {
            if byte == &b'\n' {
                pos.y += 1;
                pos.x = 0;
            } else {
                self.set_tile(vram, pos, tileset, TileSetting::from_raw(*byte as u16 - 32));
                pos.x += 1;
                if pos.x >= 30 {
                    break;
                }
            }
        }
    }

    fn fill(&mut self, vram: &mut VRamManager, tileset: &TileSet<'_>, tile_index: u16) {
        for x in 0..30 {
            for y in 0..20 {
                self.set_tile(
                    vram,
                    Vector2D { x: x, y: y },
                    tileset,
                    TileSetting::from_raw(tile_index),
                );
            }
        }
    }
}
