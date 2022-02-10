use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

use crate::song::Instruction;
use crate::song::Song;

fn play_segment(instructions: &Vec<Instruction>, tempo: f64, out: &Sink) {
    let mut notes = vec![];
    let mut note_duration = 0.0;
    let mut bending = false;
    let volume: f32 = 0.20;

    for instruction in instructions.iter() {
        // TODO: maybe handle a bend instruction that isn't between notes
        match instruction {
            Instruction::SetNoteOptions(o) => {
                note_duration = (o.duration as f64 / 36.0) * tempo;
            }
            Instruction::PlayNote(n) => {
                if bending {
                    bending = false;
                    // Just discard the start note
                    notes.pop().unwrap();
                    notes.push(
                        SineWave::new(*n)
                            .take_duration(Duration::from_secs_f64(note_duration))
                            .amplify(volume),
                    );
                } else {
                    notes.push(
                        SineWave::new(*n)
                            .take_duration(Duration::from_secs_f64(note_duration))
                            .amplify(volume),
                    );
                }
            }
            Instruction::Rest => {
                notes.push(
                    SineWave::new(0.0)
                        .take_duration(Duration::from_secs_f64(note_duration))
                        .amplify(0.0),
                );
            }
            Instruction::End => {}
            Instruction::Bend => {
                bending = true;
            }
        }
    }

    for note in notes.into_iter() {
        out.append(note);
    }
}

pub fn play_song(song: &Song) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink2 = Sink::try_new(&stream_handle).unwrap();
    let sink3 = Sink::try_new(&stream_handle).unwrap();

    for segment in song.segments.iter() {
        play_segment(&segment.square2, segment.tempo, &sink);
        play_segment(&segment.square1, segment.tempo, &sink2);
        play_segment(&segment.triangle, segment.tempo, &sink3);

        // TODO: handle truncating channels and move sleeps outside the loop
        sink.sleep_until_end();
        sink2.sleep_until_end();
        sink3.sleep_until_end();
    }
}
