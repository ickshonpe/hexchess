

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Knight,
    Bishop,
    Fortress,
    General, 
    Pawn
}

impl std::fmt::Display for Piece {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Piece {
    pub fn code(self) -> char {
        use Piece::*;
        match self {
            King => 'K',
            Queen => 'Q',
            Bishop => 'B',
            Knight => 'N',
            Fortress => 'F',
            General => 'G',
            Pawn => 'P'
        }
    }
}