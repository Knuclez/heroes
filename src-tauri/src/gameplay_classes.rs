use serde::ser::{Serialize,Serializer, SerializeStruct};

struct GameManager{
    board: Board,
}

pub struct Board{
    pieces: [Row; 6],
}

pub struct Row{
    line: [PlayablePiece; 12],
}


pub struct PlayablePiece{
    name: String

}

impl Board{
    pub fn mock_board() -> Board{
        Board{pieces: [Row::row_mockup(),
                        Row::row_mockup(),
                        Row::row_mockup(),
                        Row::row_mockup(),
                        Row::row_mockup(),
                        Row::row_mockup()]}
    }

    pub fn print_first_piece(&self) {
        self.pieces[0].get_line()[0].print_name();
    }
}

impl Row{
    pub fn row_mockup() -> Row {
        Row { line: [PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(), 
            PlayablePiece::pawn_mockup(), 
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup(),
            PlayablePiece::pawn_mockup()] }
    }

    pub fn get_line(&self) -> &[PlayablePiece; 12]{
        return &self.line;
    }
}
impl PlayablePiece {
    pub fn pawn_mockup() -> PlayablePiece{
        PlayablePiece { name: format!("Pawn") }
    }

    pub fn knight_mockup() -> PlayablePiece{
        PlayablePiece { name: format!("Kinght") }
    }

    pub fn blank_mockup() -> PlayablePiece{
        PlayablePiece { name: format!("Blank") }
    }

    pub fn print_name(&self) {
        println!("This piece name is {}", self.name.clone())
    }
}


impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Board", 1)?;
        state.serialize_field("pieces", &self.pieces);
        state.end()
    }
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Row", 1)?;
        state.serialize_field("line", &self.line);
        state.end()
    }
}

impl Serialize for PlayablePiece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("PlayablePiece", 1)?;
        state.serialize_field("name", &self.name);
        state.end()
    }
}