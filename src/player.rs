use crate::song::Segment;
use crate::song::Instruction;
use crate::song::Song;

struct Note {
    value: String,
    duration: f64,
}

fn play_segment(segment: &Segment) {
    let mut notes: Vec<Note> = vec![];
    let mut note_duration = 0.0;
    let mut bending = false;

    for instruction in segment.square2.iter() {
        // TODO: maybe handle a bend instruction that isn't between notes
        match instruction {
            Instruction::SetNoteOptions(o) => {
                note_duration = o.duration * segment.tempo;
            },
            Instruction::PlayNote(n) => {
                if bending {
                    bending = false;
                    let old_note = notes.pop().unwrap();
                    notes.push(Note {
                        value: format!("{} -> {}", old_note.value, n),
                        duration: note_duration,
                    });
                } else {
                    notes.push(Note {
                        value: n.to_string(),
                        duration: note_duration,
                    });
                }
            },
            Instruction::Rest => {
                notes.push(Note {
                    value: String::from("Rest"),
                    duration: note_duration,
                });
            },
            Instruction::End => {},
            Instruction::Bend => {
                bending = true;
            },
        } 
    }

    for note in notes.iter() {
        println!("{} {}s", note.value, note.duration);
    }
}

pub fn play_song(song: &Song) {
    for segment in song.segments.iter() {
        play_segment(segment);       
    }    
}
