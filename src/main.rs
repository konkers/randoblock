use std::fs::File;
use std::path::PathBuf;

use anvil_region::position::{RegionChunkPosition, RegionPosition};
use anvil_region::provider::{FolderRegionProvider, RegionProvider};
use structopt::StructOpt;

mod types;

use types::Level;

#[derive(Debug, StructOpt)]
enum Opt {
    LevelToJSON(ConvertOpt),
    JSONToLevel(ConvertOpt),
}

#[derive(Debug, StructOpt)]
struct ConvertOpt {
    file: PathBuf,
    #[structopt(short)]
    output: Option<PathBuf>,
}

fn level_to_json(opt: &ConvertOpt) {
    let file = File::open(&opt.file).unwrap();

    let level: Level = nbt::de::from_gzip_reader(file).unwrap();
    let output = match &opt.output {
        Some(o) => o.clone(),
        None => {
            let mut o = opt.file.clone();
            o.set_extension("json");
            o
        }
    };
    let file = File::create(&output).unwrap();
    serde_json::to_writer_pretty(file, &level).unwrap();
}

fn json_to_level(opt: &ConvertOpt) {
    let file = File::open(&opt.file).unwrap();
    let level: Level = serde_json::from_reader(&file).unwrap();

    //let level: Level = nbt::de::from_gzip_reader(file).unwrap();
    let output = match &opt.output {
        Some(o) => o.clone(),
        None => {
            let mut o = opt.file.clone();
            o.set_extension("dat");
            o
        }
    };
    // println!("{:?}", output);
    let mut file = File::create(&output).unwrap();
    nbt::ser::to_gzip_writer(&mut file, &level, None).unwrap();
    //  serde_json::to_writer_pretty(file, &level).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::LevelToJSON(o) => level_to_json(&o),
        Opt::JSONToLevel(o) => json_to_level(&o),
    }
    //let mut file = File::open("void/level.dat").unwrap();

    //    let blob = Blob::from_gzip_reader(&mut file).unwrap();
    //jlet level: Level = nbt::de::from_gzip_reader(file).unwrap();
    //println!("{:#?}", level);

    let provider = FolderRegionProvider::new("void/region");
    let region_position = RegionPosition::from_chunk_position(0, 0);
    let region_chunk_position = RegionChunkPosition::from_chunk_position(0, 0);

    let mut region = provider.get_region(region_position).unwrap();

    //  println!("{:#?}", chunk_compound_tag);
}
