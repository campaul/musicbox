mod song;
mod decoder;
mod player;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use decoder::decode_song;
use player::play_song;

const SEGMENT_MEMORY_OFFSET: usize = 188432;
const SONG_HEADERS_ADDRESS: usize = 234816;
const SEGMENT_HEADERS_ADDRESS: usize = SONG_HEADERS_ADDRESS - (37 * 7);

fn load_rom(filename: &'static str) -> Result<Vec<u8>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut rom = Vec::new();

    f.read_to_end(&mut rom)?;

    Ok(rom)
}

fn main() -> io::Result<()> {
    let rom = load_rom("smb3.nes")?;

    let song = decode_song(&rom, 0, SONG_HEADERS_ADDRESS, SEGMENT_HEADERS_ADDRESS, SEGMENT_MEMORY_OFFSET);

    play_song(&song);
        
    Ok(())
}
