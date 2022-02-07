#[derive(Debug)]
pub enum Instrument {
    LongSquare,
    ShortPiano, 
    ShortSquare,
    Square,
    Wavey,
    ShortPianoEcho,
    ShortPiano2,
    Pizzicato,
}

#[derive(Debug)]
pub struct NoteOptions {
    pub instrument: Instrument,
    pub duration: f64,
}

#[derive(Debug)]
pub enum Instruction {
    SetNoteOptions(NoteOptions),
    PlayNote(String),
    Rest,
    End,
    Bend,
}

#[derive(Debug)]
pub struct Segment {
    pub tempo: f64,
    pub square2: Vec<Instruction>,
    pub square1: Vec<Instruction>,
    pub triangle: Vec<Instruction>,
    pub noise: Vec<Instruction>,
    pub dcm: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Song {
    pub loop_index: u8,
    pub segments: Vec<Segment>,
}
