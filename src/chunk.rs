use core::num;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    os::windows::prelude::OsStrExt,
    thread::available_parallelism,
};

use anyhow::{anyhow, Result};
use num_traits::PrimInt;

use crate::types::{self, BlockType, ChunkData};

#[derive(Clone, Debug)]
struct PaletteEntry {
    ty: BlockType,
    ref_cnt: u32,
}

#[derive(Clone, Debug)]
struct Palette {
    entries: Vec<PaletteEntry>,
    entry_map: BTreeMap<BlockType, usize>,
}

impl Palette {
    fn new() -> Palette {
        Palette {
            entries: Vec::new(),
            entry_map: BTreeMap::new(),
        }
    }

    fn new_with_block(block: BlockType, ref_cnt: u32) -> Palette {
        let entries = vec![PaletteEntry {
            ty: block.clone(),
            ref_cnt,
        }];
        let mut entry_map = BTreeMap::new();
        entry_map.insert(block, 0);

        Palette { entries, entry_map }
    }

    fn inc_block(&mut self, block: &BlockType) -> u16 {
        match self.entry_map.get(block) {
            Some(index) => {
                self.entries[*index].ref_cnt += 1;
                *index as u16
            }
            None => {
                let index = self.entries.len();
                self.entries.push(PaletteEntry {
                    ty: block.clone(),
                    ref_cnt: 1,
                });
                self.entry_map.insert(block.clone(), index);

                index as u16
            }
        }
    }

    fn dec_block(&mut self, index: u16) {
        let index = index as usize;
        assert!(index < self.entries.len());
        assert!(self.entries[index].ref_cnt > 0);
        self.entries[index].ref_cnt -= 1;
    }
}

#[derive(Clone, Debug)]
struct Section {
    biomes: Vec<String>,
    blocks: Option<Vec<u16>>,
    palette: Palette,
}

impl Section {
    fn new() -> Section {
        Section {
            biomes: vec!["minecraft:plains".to_string()],
            blocks: None,
            palette: Palette::new_with_block(
                BlockType {
                    name: "minecraft:air".to_string(),
                    properties: BTreeMap::new(),
                },
                4096,
            ),
        }
    }

    fn from_data(data: &types::Section) -> Section {
        match &data.block_states {
            None => Self::new(),
            Some(states) => match &states.data {
                None => Self::new(),
                Some(block_data) => {
                    let blocks = Self::decode_data(states.palette.len(), &block_data);
                    let palette =
                        (&blocks)
                            .into_iter()
                            .fold(Palette::new(), |mut palette, index| {
                                let block_type = &states.palette[*index as usize];
                                palette.inc_block(block_type);
                                palette
                            });

                    Section {
                        biomes: data.biomes.palette.clone(),
                        blocks: Some(blocks),
                        palette,
                    }
                }
            },
        }
    }

    fn decode_data(num_palette_entries: usize, data: &Vec<i64>) -> Vec<u16> {
        let num_bits = Self::num_data_bits(num_palette_entries);
        let mut decoded = Vec::with_capacity(4096);
        for i in 0..4096 {
            decoded.push(Self::get_data(num_bits, data, i));
        }
        decoded
    }

    // Returns the number of bits needed to encode a datum referencing a
    // a palette with `num_entries`.  A minimum of 4 will always be
    // returned.
    fn num_data_bits(num_entries: usize) -> u32 {
        let num_bits = num_entries.next_power_of_two().trailing_zeros();
        std::cmp::max(num_bits, 4)
    }

    // Algorithm for indexing into data adapted from fastanvil:
    // https://github.com/owengage/fastnbt
    fn get_data(num_bits: u32, data: &Vec<i64>, index: usize) -> u16 {
        let elements_per_long = 64 / num_bits as usize;
        let long_val = data[index / elements_per_long] as u64;
        let element_index = index % elements_per_long;

        // Extract element from long.
        let val = long_val >> (element_index * num_bits as usize);
        let val = val & (1 << num_bits) - 1;

        val as u16
    }

    fn set_data(num_bits: u32, data: &mut Vec<i64>, index: usize, val: u16) {
        let elements_per_long = 64 / num_bits as usize;
        let data_index = index / elements_per_long;
        let mut long_val = data[data_index] as u64;

        let element_bit_offset = (index % elements_per_long) * num_bits as usize;
        let mask = ((1 << num_bits) - 1) << element_bit_offset;

        long_val &= !mask;
        long_val |= (val as u64) << element_bit_offset;

        data[data_index] = long_val as i64;
    }

    fn set_block(&mut self, x: i32, y: i32, z: i32, block_type: &BlockType) {
        assert!(0 <= x && x < 16);
        assert!(0 <= y && y < 16);
        assert!(0 <= z && z < 16);

        if let None = self.blocks {
            self.blocks = Some(vec![0u16; 4096]);
        }

        let block = &mut self.blocks.as_mut().unwrap()[(y * 64 + z * 16 + x) as usize];
        self.palette.dec_block(*block);
        *block = self.palette.inc_block(block_type);
    }

    fn to_nbt(&self, y: i8) -> types::Section {
        let biomes = types::Biomes {
            palette: self.biomes.clone(),
        };
        //TODO: prune unused palette entries
        let palette = self.palette.entries.iter().map(|e| e.ty.clone()).collect();
        let data = match &self.blocks {
            None => None,
            Some(blocks) => {
                let num_bits = Self::num_data_bits(self.palette.entries.len());
                let elements_per_long = 64 / num_bits as usize;
                // rounding up division
                let num_longs = (4096 + (elements_per_long - 1)) / elements_per_long;
                let mut data = vec![0i64; num_longs];
                for i in 0..4096 {
                    Self::set_data(num_bits, &mut data, i, blocks[i]);
                }

                Some(data)
            }
        };

        types::Section {
            biomes,
            block_states: Some(types::BlockStates { palette, data }),
            sky_light: None, // Hoping that Minecraft will recalc this if missing.
            y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    data: ChunkData,
    dirty: bool,

    sections: Vec<Section>,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Chunk {
        let mut sections = Vec::new();
        for i in 0..24 {
            let y = -4 + i as i8;

            sections.push(types::Section {
                biomes: types::Biomes {
                    palette: vec!["minecraft:plains".to_string()],
                },
                block_states: Some(types::BlockStates {
                    palette: vec![BlockType {
                        name: "minecraft:air".to_string(),
                        properties: BTreeMap::new(),
                    }],
                    data: None,
                }),
                sky_light: None,
                y,
            });
        }
        Chunk {
            data: ChunkData {
                height_maps: HashMap::new(),
                structures: types::Structures {
                    references: HashMap::new(),
                    starts: HashMap::new(),
                },
                block_entities: Vec::new(),
                block_ticks: Vec::new(),
                fluid_ticks: Vec::new(),
                post_processing: vec![Vec::new(); 24],
                sections,
                data_version: 2865,
                inhabitied_time: None,
                is_light_on: Some(true),
                last_update: 0,
                x_pos: x,
                y_pos: -4,
                z_pos: z,
                status: "full".to_string(),
            },
            dirty: false,
            sections: vec![Section::new(); 24],
        }
    }

    pub fn from_data(data: ChunkData) -> Chunk {
        let mut sections = Vec::with_capacity(24);

        for i in 0..24 {
            sections.push(Section::from_data(&data.sections[i]));
        }

        Chunk {
            data,
            dirty: false,
            sections,
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block_type: &BlockType) {
        // Adjust for where the bottom of the chunk lies.
        let y = y - self.data.y_pos * 16;
        assert!(y >= 0);
        let section_index = (y as usize) / 16;
        assert!(section_index < 24);

        let section_y = y % 16;

        self.sections[section_index].set_block(x, section_y, z, block_type);
    }

    pub fn to_nbt(&self) -> ChunkData {
        let mut data = self.data.clone();
        for i in 0..24 {
            data.sections[i] = self.sections[i].to_nbt((data.y_pos + i as i32) as i8);
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_data_bits() {
        assert_eq!(4, Section::num_data_bits(1));
        assert_eq!(4, Section::num_data_bits(2));
        assert_eq!(4, Section::num_data_bits(3));
        assert_eq!(4, Section::num_data_bits(4));
        assert_eq!(4, Section::num_data_bits(5));
        assert_eq!(4, Section::num_data_bits(8));
        assert_eq!(4, Section::num_data_bits(9));
        assert_eq!(4, Section::num_data_bits(16));
        assert_eq!(5, Section::num_data_bits(17));
    }
}
