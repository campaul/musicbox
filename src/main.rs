mod decoder;
mod player;
mod song;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use decoder::decode_song;
use player::play_song;

const SEGMENT_MEMORY_OFFSET: usize = 188432;
const SONG_HEADERS_ADDRESS: usize = 234816;
const SEGMENT_HEADERS_ADDRESS: usize = SONG_HEADERS_ADDRESS - (37 * 7);

fn load_rom(filename: String) -> Result<Vec<u8>, std::io::Error> {
    let mut f = File::open(filename.as_str())?;
    let mut rom = Vec::new();

    f.read_to_end(&mut rom)?;

    Ok(rom)
}

fn print_help() -> io::Result<()> {
    println!("Usage: musicbox <filename> <track_number>");
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return print_help();
    }

    let filename = &args[1];
    let track = &args[2].parse::<usize>().unwrap();

    println!("Loading track {} from {}\n", track, filename);

    let rom = load_rom(filename.clone())?;
    let song = decode_song(
        &rom,
        *track,
        SONG_HEADERS_ADDRESS,
        SEGMENT_HEADERS_ADDRESS,
        SEGMENT_MEMORY_OFFSET,
    );

    play_song(&song);

    Ok(())
}
