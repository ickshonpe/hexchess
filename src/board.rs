use crate::pieces::Piece;

pub const HEX_COUNT: usize = 91;
pub const RANK_LENGTHS: [usize; 11] = [
    6,
    7,
    8,
    9,
    10,
    11,
    10,
    9,
    8,
    7,
    6
];
pub const OFFS: [f32; 11] = [
    0., 
    -1.,
    -2.,
    -3.,
    -4.,
    -5.,
    -4.,
    -3.,
    -2.,
    -1.,
    0.
];

pub const BLACK_QUEENING_HEXES: std::ops::RangeInclusive<usize> = 84..=90;
pub const WHITE_QUEENING_HEXES: std::ops::RangeInclusive<usize> = 0..=5;

pub fn is_queening_hex(hex: usize, player: Player) -> bool {
    match player {
        Player::White => {
            WHITE_QUEENING_HEXES.contains(&hex)
        },
        Player::Black => {
            BLACK_QUEENING_HEXES.contains(&hex)
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub fn opposite(self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
}
pub enum Direction {
    UpLeft = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    DownLeft = 4,
    Left = 5
}

pub type Hexes = [Option<(Player, Piece)>; HEX_COUNT];

pub struct Board {
    pub current_turn: Player,
    pub hexes: [Option<(Player, Piece)>; HEX_COUNT],
    //neighbours: [[usize; 6]; HEX_COUNT],
    pub positions: [[f32; 2]; HEX_COUNT],
    pub threats: [u8; HEX_COUNT]
    //ranks: [usize; HEX_COUNT],
    //files: [usize; HEX_COUNT]
}

impl Board {
    pub fn new() -> Board {
        let mut b = Board {
            current_turn: Player::White,
            hexes: place_pieces(),
            positions: crate::geometry::create_board_positions(),
            threats: [0; HEX_COUNT]
        };
        let threats = crate::logic::count_threats(&b);
        b.threats = threats;
        b
    }
}

fn get_rank(hex: usize) -> usize {
    let mut cursor = hex;
    for (rank, len) in RANK_LENGTHS.iter().enumerate() {
        if cursor < *len {
            return rank;
        } else {
            cursor -= len;
        }
    }
    panic!("invalid board index: {}", hex);
}

fn validate_hex(hex: usize) {
    assert!(hex < HEX_COUNT, "invalid board index: {}", hex);
}

fn on_left_edge(hex: usize) -> bool {
    let file = get_file(hex);
    file == 0
}

fn on_right_edge(hex: usize) -> bool {
    let file = get_file(hex);
    let rank = get_rank(hex);
    file == RANK_LENGTHS[rank] - 1
}

fn on_top_rank(hex: usize) -> bool {
    get_rank(hex) == 0
}

fn on_bottom_rank(hex: usize) -> bool {
    get_rank(hex) == 10
}





fn get_file(hex: usize) -> usize {
    let mut cursor = hex;
    for len in RANK_LENGTHS.iter() {
        if cursor < *len {
            return cursor;
        } else {
            cursor -= len;
        }
    }
    panic!("invalid board index: {}", hex);
}

pub fn left(hex: usize) -> Option<usize> {
    if on_left_edge(hex) {
        return None;
    }
    Some(hex - 1)
}

pub fn right(hex: usize) -> Option<usize> {
    if on_right_edge(hex) {
        return None;
    }
    Some(hex + 1)
}

pub fn up_left(hex: usize) -> Option<usize> {
    match hex {
        0..=5 => None,
        7..=12 => Some(hex - 7),
        14..=20 => Some(hex - 8),
        22..=29 => Some(hex - 9),
        31..=39 => Some(hex - 10),
        41..=50 => Some(hex - 11),
        51..= 60 => Some(hex - 11),
        61..= 69 => Some(hex - 10),
        70..= 77 => Some(hex - 9),
        78..= 84 => Some(hex - 8),
        85..= 90 => Some(hex - 7),
        _ => None
    }
}

pub fn adjacent(hex: usize) -> Vec<usize> {
    let hs = [
        up_left(hex),
        up_right(hex),
        right(hex),
        down_right(hex),
        down_left(hex),
        left(hex)
    ];
    hs.iter().filter_map(|h| *h).collect()
}

pub fn up_right(hex: usize) -> Option<usize> {
    match hex {
        0..= 5 => None,
        6..= 11 => Some(hex - 6),
        13..= 19 => Some(hex - 7),
        21..= 28 => Some(hex - 8),
        30..= 38 => Some(hex - 9),
        40..= 49 => Some(hex - 10),
        51..= 60 => Some(hex - 10),
        61..= 69 => Some(hex - 9),
        70..= 77 => Some(hex - 8),
        78..= 84 => Some(hex - 7),
        85..= 90 => Some(hex - 6),
        _ => None
    }
}

pub fn down_left(hex: usize) -> Option<usize> {
    match hex {
        0..=5 => Some(hex + 6),
        6..=12 => Some(hex + 7),
        13..=20 => Some(hex + 8),
        21..=29 => Some(hex + 9),
        30..=39 => Some(hex + 10),
        41..=50 => Some(hex + 10),
        52..= 60 => Some(hex + 9),
        62..= 69 => Some(hex + 8),
        71..= 77 => Some(hex + 7),
        79..= 84 => Some(hex + 6),
        _ => None
    }
}

pub fn down_right(hex: usize) -> Option<usize> {
    match hex {
        0..= 5 => Some(hex + 7),
        6..= 12 => Some(hex + 8),
        13..= 20 => Some(hex + 9),
        21..= 29 => Some(hex + 10),
        30..= 39 => Some(hex + 11),
        40..= 49 => Some(hex + 11),
        51..= 59 => Some(hex + 10),
        61..= 68 => Some(hex + 9),
        70..= 76 => Some(hex + 8),
        78..= 83 => Some(hex + 7),
        _ => None
    }
}

pub fn has_white_piece(hexes: Hexes, hex: usize) -> bool {
    match hexes[hex] {
        Some((Player::White, _)) => true,
        _ => false
    }
}

pub fn has_black_piece(hexes: Hexes, hex: usize) -> bool {
    match hexes[hex] {
        Some((Player::Black, _)) => true,
        _ => false
    }
}

pub fn has_colored_piece(hexes: Hexes, hex: usize, color: Player) -> bool {
    match hexes[hex] {
        Some((c, _)) if c == color => true,
        _ => false
    }
}


fn place_pieces() -> [Option<(Player, Piece)>; HEX_COUNT] {
    use Piece::*;
    use maplit::*;
    
    let piece_map: std::collections::HashMap<usize, Piece> = hashmap!{
        0 => Pawn,
        1 => Bishop,
        2 => Queen,
        3 => King,
        4 => Bishop,
        5 => Pawn,
        7 => Pawn,
        8 => Knight,
        9 => Fortress,
        10 => Knight,
        11 => Pawn,
        15 => Pawn,
        16 => General,
        17 => General,
        18 => Pawn,
        24 => Pawn,
        25 => Pawn,
        26 => Pawn
    };
    let mut pieces = [None; HEX_COUNT];
    for idx in 0..(HEX_COUNT/2) {
        let piece = piece_map.get(&idx).cloned();
        pieces[idx] = piece.map(|piece| (Player::Black, piece));
        pieces[HEX_COUNT - idx - 1] = piece.map(|piece| (Player::White, piece));
    }
    pieces
}


fn alt_place_pieces() -> [Option<(Player, Piece)>; HEX_COUNT] {
    use Piece::*;
    use maplit::*;
    
    let piece_map: std::collections::HashMap<usize, Piece> = hashmap!{
        0 => Knight,
        1 => Bishop,
        2 => Queen,
        3 => King,
        4 => Bishop,
        5 => Knight,
        6 => Pawn,
        7 => Pawn,
        8 => Pawn,
        9 => Pawn,
        10 => Pawn,
        11 => Pawn,
        12 => Pawn,
        23 => General,
        25 => Fortress,
        27 => General
    };
    let mut pieces = [None; HEX_COUNT];
    for idx in 0..(HEX_COUNT/2) {
        let piece = piece_map.get(&idx).cloned();
        pieces[idx] = piece.map(|piece| (Player::Black, piece));
        pieces[HEX_COUNT - idx - 1] = piece.map(|piece| (Player::White, piece));
    }
    pieces
}

