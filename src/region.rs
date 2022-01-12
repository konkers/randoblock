use std::io::{prelude::*, Cursor, SeekFrom};

use anyhow::{anyhow, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::ZlibDecoder;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::chunk::Chunk;
use crate::types::{BlockType, ChunkData};

pub struct Region {
    x: i32,
    z: i32,

    chunks: Vec<Option<Chunk>>,
}

struct ChunkLocation {
    offset: u64,
    size: u64,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
enum CompressionType {
    GZIP = 1,
    ZLIB = 2,
    UNCOMPRESSED = 3,
}

impl Region {
    pub fn new(x: i32, z: i32) -> Region {
        Region {
            x,
            z,
            chunks: vec![None; 1024],
        }
    }

    pub fn from_reader(x: i32, z: i32, mut r: impl Read + Seek) -> Result<Region> {
        r.seek(SeekFrom::Start(0))?;

        let mut chunk_locations = Vec::new();
        let mut chunk_timestamps = Vec::new();

        for _ in 0..1024 {
            let offset = r.read_u24::<BigEndian>()?;
            let size = r.read_u8()?;
            chunk_locations.push(ChunkLocation {
                offset: offset as u64 * 4096,
                size: size as u64 * 4096,
            });
        }

        for _ in 0..1024 {
            let timestamp = r.read_u32::<BigEndian>()?;
            chunk_timestamps.push(timestamp);
        }

        let mut chunks = vec![None; 1024];

        for i in 0..1024 {
            // Chunks with zero size are empty.
            if chunk_locations[i].size == 0 {
                continue;
            }

            // println!("chunk {}", i);

            let chunk = Self::load_chunk(&mut r, i, &chunk_locations[i])?;
            chunks[i] = Some(chunk);
        }

        Ok(Region { x, z, chunks })
    }

    fn load_chunk<R: Read + Seek>(r: &mut R, index: usize, loc: &ChunkLocation) -> Result<Chunk> {
        r.seek(SeekFrom::Start(loc.offset))?;
        let length = r.read_u32::<BigEndian>()?;
        if (length + 4) as u64 > loc.size {
            return Err(anyhow!("Chunk #{} has inconsitant size", index));
        }

        let compression_type_raw = r.read_u8()?;
        let compression_type = CompressionType::from_u8(compression_type_raw)
            .ok_or(anyhow!("Unknown compression type {}", compression_type_raw))?;

        let data: ChunkData = match compression_type {
            CompressionType::ZLIB => nbt::from_zlib_reader(r)?,
            _ => {
                return Err(anyhow!(
                    "Anyhow {:?} chunk compression unsupported",
                    compression_type
                ));
            }
        };
        if index == 0 {
            println!("{:#?}", Chunk::from_data(data.clone()));
        }

        Ok(Chunk::from_data(data))
    }

    fn round_up_to(val: u64, div: u64) -> u64 {
        match val % div {
            0 => val,
            x => val + div - x,
        }
    }

    pub fn to_writer<W: Write + Seek>(&self, w: &mut W, timestamp: u32) -> Result<()> {
        let mut locations = Vec::new();
        let mut cur_sector = 2;

        for i in 0..1024 {
            let offset = cur_sector * 4096;
            match &self.chunks[i] {
                None => locations.push(ChunkLocation { offset: 0, size: 0 }),
                Some(chunk) => {
                    // seek past the length header
                    w.seek(SeekFrom::Start(offset + 4))?;

                    // Write compression type byte
                    w.write_u8(CompressionType::ZLIB.to_u8().unwrap())?;

                    // Write chunk data
                    let chunk_data = chunk.to_nbt();
                    nbt::to_zlib_writer(w, &chunk_data, None)?;

                    // Write lenghth field at beginning
                    let cur_offset = w.stream_position()?;
                    let len = cur_offset - offset - 4;
                    w.seek(SeekFrom::Start(offset))?;
                    w.write_u32::<BigEndian>(len as u32)?;

                    locations.push(ChunkLocation {
                        offset: offset,
                        size: Self::round_up_to(len + 4, 4096),
                    });
                }
            }
        }

        // Now that we know the sizes of all the chunks, we can write out the
        // header
        w.seek(SeekFrom::Start(0))?;
        for i in 0..1024 {
            w.write_u24::<BigEndian>((locations[i].offset / 4096) as u32);
            w.write_u8((locations[i].size / 4096) as u8);
        }
        for i in 0..1024 {
            w.write_u32::<BigEndian>(timestamp);
        }

        Ok(())
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block_type: &BlockType) {
        assert!(0 <= x && x <= 512);
        assert!(0 <= z && z <= 512);

        let chunk_index = ((x % 32) + (z % 32) * 32) as usize;
        assert!(chunk_index < 1024);

        let chunk_x = (self.x * 512 + x) / 32;
        let chunk_z = (self.z * 512 + z) / 32;
        self.chunks[chunk_index]
            .get_or_insert(Chunk::new(chunk_x, chunk_z))
            .set_block(x % 16, y, z % 15, block_type);
    }
}
