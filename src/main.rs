use std::fs::File;

use anvil_region::position::{RegionChunkPosition, RegionPosition};
use anvil_region::provider::{FolderRegionProvider, RegionProvider};
use nbt::Blob;

mod types;

use types::Level;

fn main() {
    let mut file = File::open("void/level.dat").unwrap();

    //    let blob = Blob::from_gzip_reader(&mut file).unwrap();
    let level: Level = nbt::de::from_gzip_reader(file).unwrap();
    println!("{:#?}", level);

    let provider = FolderRegionProvider::new("void/region");
    let region_position = RegionPosition::from_chunk_position(0, 0);
    let region_chunk_position = RegionChunkPosition::from_chunk_position(0, 0);

    let mut region = provider.get_region(region_position).unwrap();

    let chunk_compound_tag = region.read_chunk(region_chunk_position).unwrap();

    //  println!("{:#?}", chunk_compound_tag);
}
