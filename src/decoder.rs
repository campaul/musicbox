use crate::song::Instruction;
use crate::song::Instrument;
use crate::song::NoteOptions;
use crate::song::Segment;
use crate::song::Song;

fn decode_tempo(value: u8) -> f64 {
    match value {
        0x00 => 0.275, // random guess
        0x10 => 0.250,
        0x20 => 0.233,
        0x30 => 0.2,
        0x40 => 0.166,
        0x50 => 0.150,
        0x60 => 0.133,
        0x70 => 0.115,
        0x80 => 0.100,
        0x90 => 0.066,
        _ => panic!("Invalid tempo: {}", value),
    }
}

fn decode_note(value: u8) -> f32 {
    match value {
        0x02 => 69.30,   // C#2
        0x04 => 73.42,   // D2
        0x06 => 77.78,   // D#2
        0x08 => 82.41,   // E2,
        0x0A => 87.31,   // F2
        0x0C => 92.50,   // F#2
        0x0E => 98.00,   // G2
        0x10 => 103.83,  // G#2
        0x12 => 110.00,  // A2
        0x14 => 116.54,  // A#2
        0x16 => 123.46,  // B2
        0x18 => 130.81,  // C3
        0x1A => 138.59,  // C#3
        0x1C => 146.83,  // D3
        0x1E => 155.56,  // D#3
        0x20 => 164.81,  // E3
        0x22 => 174.61,  // F3
        0x24 => 185.00,  // F#3
        0x26 => 196.00,  // G3
        0x28 => 207.65,  // G#3
        0x2A => 220.00,  // A3
        0x2C => 233.08,  // A#3
        0x2E => 246.94,  // B3
        0x30 => 261.63,  // C4
        0x32 => 277.18,  // C#4
        0x34 => 293.66,  // D4
        0x36 => 331.13,  // D#4
        0x38 => 329.63,  // E4
        0x3A => 349.23,  // F4
        0x3C => 369.99,  // F#4
        0x3E => 392.00,  // G4
        0x40 => 415.30,  // G#4
        0x42 => 440.00,  // A4
        0x44 => 466.15,  // A#4
        0x46 => 493.88,  // B4
        0x48 => 523.25,  // C5
        0x4A => 554.37,  // C#5
        0x4C => 587.33,  // D5
        0x4E => 622.25,  // D#5
        0x50 => 659.25,  // E5
        0x52 => 698.46,  // F5
        0x54 => 739.99,  // F#5
        0x56 => 783.99,  // G5
        0x58 => 830.61,  // G#5
        0x5A => 880.00,  // A5
        0x5C => 932.33,  // A#5
        0x5E => 987.77,  // B5
        0x60 => 1046.50, // C6
        0x62 => 1108.73, // C#6
        0x64 => 1174.66, // D6
        0x66 => 1244.51, // D#6
        0x68 => 1318.51, // E6
        0x6A => 1396.91, // F6
        0x6C => 1479.98, // F#6
        0x6E => 1567.98, // G6
        0x70 => 1661.22, // G#6
        0x72 => 1760.00, // A6
        0x74 => 1864.66, // A#6
        0x76 => 1975.72, // B6
        0x78 => 2093.00, // C7
        0x7A => 2217.46, // C#7
        0x7C => 2349.32, // D7
        0x7E => 0.0,     // rest,
        _ => panic!("Unknown note: {}", value),
    }
}

fn decode_note_length(value: u8) -> f64 {
    match value {
        0x0 => 0.5,
        0x1 => 0.55,
        0x2 => 0.66,
        0x3 => 0.66,
        0x4 => 1.0,
        0x5 => 1.5,
        0x6 => 1.33,
        0x7 => 0.75,
        0x8 => 2.0,
        0x9 => 3.0,
        0xa => 4.0,
        0xb => 6.0,
        0xc => 8.0,
        0xd => 3.33,
        0xe => 0.33,
        0xf => panic!("Unsupported note length: 0xf"),
        _ => panic!("Invalid note length: {}", value),
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
        _ => panic!("Unknown instrument: {}", patch_number),
    };

    return NoteOptions {
        instrument: patch,
        duration: note_length,
    };
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

    fn to_song(
        &self,
        rom: &Vec<u8>,
        segment_headers_index: usize,
        segment_memory_offset: usize,
    ) -> Song {
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
            loop_index: song_index[self.loop_index as usize] / 7,
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
        for i in 0..37 {
            let start = segment_headers_index + (i * 7);
            let end = start + 7;
            let segment_bytes = &rom[start..end];
            segment_headers.push(SegmentHeader::new(
                segment_bytes.try_into().expect("wrong length"),
            ));
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

pub fn decode_song(
    rom: &Vec<u8>,
    song_index: usize,
    song_headers_address: usize,
    segment_headers_address: usize,
    segment_memory_offset: usize,
) -> Song {
    let song_headers = SongHeader::load(&rom, song_headers_address);
    song_headers[song_index].to_song(&rom, segment_headers_address, segment_memory_offset)
}
