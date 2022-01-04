use std::{fs::File, os::unix::prelude::MetadataExt, time::Instant};

use clausewitz_parser::clausewitz::root::par_root;
use memmap::Mmap;

fn main() {
    let filename = "/home/michael/Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/gamestate_large2";
    let file = File::open(filename).expect("File not found");
    let mmap = unsafe { Mmap::map(&file).expect(&format!("Error mapping file {:?}", file)) };

    let str = std::str::from_utf8(&mmap[..]).unwrap();

    let s1 = Instant::now();
    let prepared_input = str.replace("\n}\n", "\n}\n#");
    println!("{}", s1.elapsed().as_millis());

    let start = Instant::now();
    let _ = par_root(prepared_input.as_str());

    let end = start.elapsed();

    let size_in_bytes = file.metadata().unwrap().size();
    let speed = (size_in_bytes as u128 / end.as_millis()) * 1000;

    println!(
        "{:?}MB/s, took {} seconds.",
        speed as f32 / 1000000 as f32,
        end.as_secs()
    );
}
