use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

use crate::song::Instruction;
use crate::song::Segment;
use crate::song::Song;

fn play_segment(segment: &Segment, out: &Sink) {
    let mut notes = vec![];
    let mut note_duration = 0.0;
    let mut bending = false;
    let volume: f32 = 0.20;

    for instruction in segment.square2.iter() {
        // TODO: maybe handle a bend instruction that isn't between notes
        match instruction {
            Instruction::SetNoteOptions(o) => {
                note_duration = o.duration * segment.tempo;
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

    for segment in song.segments.iter() {
        play_segment(segment, &sink);
    }

    sink.sleep_until_end();
}
