use crate::song::Instruction;
use crate::song::NoteOptions;
use crate::song::Instrument;
use crate::song::Segment;
use crate::song::Song;

fn decode_tempo(value: u8) -> f64 {
    match value {
        0x10 => 0.250,
        0x20 => 0.233,
        0x30 => 0.2,
        0x40 => 0.166,
        0x50 => 0.150,
        0x60 => 0.133,
        0x70 => 0.115,
        0x80 => 0.100,
        0x90 => 0.066,
        _ => 0.0,
    }
}

fn decode_note(value: u8) -> String {
    String::from(match value {
        0x02 => "c#2",
        0x04 => "D2",
        0x06 => "D#2",
        0x08 => "E2",
        0x0A => "F2",
        0x0C => "F#2",
        0x0E => "G2",
        0x10 => "G#2",
        0x12 => "A2",
        0x14 => "A#2",
        0x16 => "B2",
        0x18 => "C3",
        0x1A => "C#3",
        0x1C => "D3",
        0x1E => "D#3",
        0x20 => "E3",
        0x22 => "F3",
        0x24 => "F#3",
        0x26 => "G3",
        0x28 => "G#3",
        0x2A => "A3",
        0x2C => "A#3",
        0x2E => "B3",
        0x30 => "C4",
        0x32 => "C#4",
        0x34 => "D4",
        0x36 => "D#4",
        0x38 => "E4",
        0x3A => "F4",
        0x3C => "F#4",
        0x3E => "G4",
        0x40 => "G#4",
        0x42 => "A4",
        0x44 => "A#4",
        0x46 => "B4",
        0x48 => "C5",
        0x4A => "C#5",
        0x4C => "D5",
        0x4E => "D#5",
        0x50 => "E5",
        0x52 => "F5",
        0x54 => "F#5",
        0x56 => "G5",
        0x58 => "G#5",
        0x5A => "A5",
        0x5C => "A#5",
        0x5E => "B5",
        0x60 => "C6",
        0x62 => "C#6",
        0x64 => "D6",
        0x66 => "D#6",
        0x68 => "E6",
        0x6A => "F6",
        0x6C => "F#6",
        0x6E => "G6",
        0x70 => "G#6",
        0x72 => "A6",
        0x74 => "A#6",
        0x76 => "B6",
        0x78 => "C7",
        0x7A => "C#7",
        0x7C => "D7",
        0x7E => "blank",
        _ => "Unknown Note",
    })
}

fn decode_note_length(value: u8) -> f64 {
    match value {
        0x0 => 0.5,
        0x1 => 0.55,
        0x2	=> 0.66,
        0x3	=> 0.66,
        0x4	=> 1.0,
        0x5 => 1.5,
        0x6 => 1.33,
        0x7 => 0.75,
        0x8 => 2.0,
        0x9	=> 3.0,
        0xa	=> 4.0,
        0xb	=> 6.0,
        0xc	=> 8.0,
        0xd	=> 3.33,
        0xe	=> 0.33,
        0xf	=> panic!("Unsupported note length"),
        _ => panic!("Invalid note length"),
    }
}

fn decode_note_options(value: u8) -> NoteOptions {
    let patch_number = (value >> 4) - 8;
    let note_length = decode_note_length(value << 4 >> 4);

    let patch = match patch_number {
        0 => Instrument::LongSquare,
        1 => Instrument::ShortPiano,
        2 => Instrument::ShortSquare,
        3 => Instrument::Square,
        4 => Instrument::Wavey,
        5 => Instrument::ShortPianoEcho,
        6 => Instrument::ShortPiano2,
        7 => Instrument::Pizzicato,
        _ => panic!("Unknown Instrument"),
    };

    return NoteOptions {
        instrument: patch,
        duration: note_length,
    }
}

fn decode_channel(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut instructions = vec![];

    for value in bytes.into_iter() {
        if value == 0x00 as u8 {
            instructions.push(Instruction::End);
        } else if value >= 0x01 && value <= 0x7d {
            instructions.push(Instruction::PlayNote(decode_note(value)))
        } else if value == 0x7e {
            instructions.push(Instruction::Rest);
        } else if value >= 0x80 && value <= 0xfe {
            instructions.push(Instruction::SetNoteOptions(decode_note_options(value)));
        } else if value == 0xff {
            instructions.push(Instruction::Bend);
        }
    }

    return instructions;
}

#[derive(Debug)]
struct SongHeader {
    start_index: u8,
    end_index: u8,
    loop_index: u8,
}

impl SongHeader {

    fn load(rom: &Vec<u8>, starts_index: usize) -> Vec<SongHeader> {
        // TODO: take this as arg
        let ends_index = 234816 + 12;
        let loops_index = 234816 + 24;

        let starts = rom[starts_index..starts_index + 12].to_vec();
        let ends = rom[ends_index..ends_index + 12].to_vec();
        let loops = rom[loops_index..loops_index + 12].to_vec();
    
        let mut songs: Vec<SongHeader> = vec![];

        for i in 0..starts.len() {
            songs.push(SongHeader {
                start_index: starts[i],
                end_index: ends[i],
                loop_index: loops[i],
            });
        }

        return songs;
    }

    fn to_song(&self, rom: &Vec<u8>, segment_headers_index: usize, segment_memory_offset: usize) -> Song {
        let segment_headers = SegmentHeader::load(&rom, segment_headers_index);
        let song_to_header_index_address = segment_headers_index - 45;
        let song_index = &rom[song_to_header_index_address..song_to_header_index_address + 45];

        let mut segments = vec![];

        for i in self.start_index..self.end_index + 1 {
            let segment_index = song_index[i as usize] / 7;
            let segment_header = &segment_headers[segment_index as usize];
            let segment = segment_header.to_segment(rom, segment_memory_offset);
            segments.push(segment);
        }

        Song {
            loop_index: song_index[self.loop_index as usize] /7,
            segments: segments,
        }
    }

}

#[derive(Debug)]
struct SegmentHeader {
    tempo: u8,
    address: u16,
    triangle_offset: u8,
    square1_offset: u8,
    noise_offset: u8,
    dcm_offset: u8,
}

impl SegmentHeader {
    fn new(data: [u8; 7]) -> SegmentHeader {
        let mut address: u16 = data[2] as u16;
        address = address << 8;
        address = address + data[1] as u16;

        SegmentHeader {
            tempo: data[0],
            address: address,
            triangle_offset: data[3],
            square1_offset: data[4],
            noise_offset: data[5],
            dcm_offset: data[6],
        }
    }

    fn load(rom: &Vec<u8>, segment_headers_index: usize) -> Vec<SegmentHeader> {
        let mut segment_headers: Vec<SegmentHeader> = vec![];
        for i in 0..36 {
            let start = segment_headers_index + (i * 7);
            let end = start + 7;
            let segment_bytes = &rom[start..end];
            segment_headers.push(SegmentHeader::new(segment_bytes.try_into().expect("wrong length")));
        }
        return segment_headers;
    }

    fn to_segment(&self, rom: &Vec<u8>, segment_memory_offset: usize) -> Segment {
        let start_address = self.address as usize + segment_memory_offset;

        let mut square2_data: Vec<u8> = vec![];
        let mut i = 0;

        loop {
            let d = rom[start_address + i];
            square2_data.push(d);
            i = i + 1;

            if d == 0 {
                break;
            }
        }

        Segment {
            tempo: decode_tempo(self.tempo),
            square2: decode_channel(square2_data),
            square1: vec![],
            triangle: vec![],
            noise: vec![],
            dcm: vec![],
        }
    }
}

pub fn decode_song(rom: &Vec<u8>, song_index: usize, song_headers_address: usize, segment_headers_address: usize, segment_memory_offset: usize) -> Song {
    let song_headers = SongHeader::load(&rom, song_headers_address);
    song_headers[song_index].to_song(&rom, segment_headers_address, segment_memory_offset)
}
