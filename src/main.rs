use std::path::PathBuf;
use std::{fs::File, time::SystemTime};

use structopt::StructOpt;

mod chunk;
mod region;
mod types;

use region::Region;
use types::{BlockType, Level};

#[derive(Debug, StructOpt)]
enum Opt {
    LevelToJSON(ConvertOpt),
    JSONToLevel(ConvertOpt),
    Region(FileOpt),
    NewRegion(FileOpt),
}

#[derive(Debug, StructOpt)]
struct FileOpt {
    file: PathBuf,
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

fn region(opt: &FileOpt) {
    let file = File::open(&opt.file).unwrap();
    let _ = Region::from_reader(0, 0, file).unwrap();
}

fn new_region(opt: &FileOpt) {
    let mut region = Region::new(0, 0);
    region.set_block(0, 0, 0, &BlockType::new("minecraft:jungle_planks"));
    let mut file = File::create(&opt.file).unwrap();
    region
        .to_writer(
            &mut file,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        )
        .unwrap();
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::LevelToJSON(o) => level_to_json(&o),
        Opt::JSONToLevel(o) => json_to_level(&o),
        Opt::Region(o) => region(&o),
        Opt::NewRegion(o) => new_region(&o),
    }
}
