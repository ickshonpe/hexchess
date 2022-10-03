use crate::board::*;
use crate::pieces::Piece::*;

fn is_valid_target(b: &Board, color: Player, t: usize) -> bool {
    b.hexes[t].is_none()
    || has_colored_piece(b.hexes, t, color.opposite())
    && is_vulnerable(b, t)
}

pub fn is_vulnerable(b: &Board, t: usize) -> bool {
    if let Some((_, piece)) = b.hexes[t] {
        if piece == Fortress {
            return b.threats[t] > 1;
        }
    }
    true
}

pub fn find_valid_moves(board: &Board, hex: usize) -> Vec<usize> {
    match board.hexes[hex] {
        Some((color, piece)) => {
            match piece {
                King => {
                    get_king_moves(board.hexes, hex)
                },
                Bishop => {
                    get_bishop_moves(board.hexes, hex)
                }
                Queen => {
                    get_queen_moves(board.hexes, hex)
                },
                Knight => {
                    get_knight_moves(board.hexes, hex)
                },
                Fortress => {
                    get_fortress_moves(board.hexes, hex)
                },
                General => {
                    get_general_moves(board.hexes,hex)
                }, 
                Pawn => {
                    if color == Player::White {
                        let left = 
                            up_left(hex).and_then(|t|
                                    if board.hexes[t].is_none()
                                    || has_colored_piece(board.hexes, t, color.opposite()) {
                                        Some(t)
                                    } else {
                                        None
                                    }
                            );
                        let right = 
                            up_right(hex).and_then(|t|
                                    if board.hexes[t].is_none()
                                    || has_colored_piece(board.hexes, t, color.opposite()) {
                                        Some(t)
                                    } else {
                                        None
                                    }
                            );
                        let ul = if let Some(t) = up_left(hex) {
                            board.hexes[t].is_none()
                        } else { false };
                        let ur = if let Some(t) = up_right(hex) {
                            board.hexes[t].is_none()
                        } else { false };
                        let u: Option<usize> = up_right(hex).and_then(|t| up_left(t));
                        let up =
                            if ul && ur {
                                if let Some(u) = u {
                                    if board.hexes[u].is_none() {
                                        Some(u)
                                    } else { None }
                                } else { None }
                            } else { None };
                        
                        [left, right, up].iter().filter_map(|h| *h).collect()
                    } else {
                        let left = 
                            down_left(hex).and_then(|t|
                                    if board.hexes[t].is_none()
                                    || has_colored_piece(board.hexes, t, color.opposite()) {
                                        Some(t)
                                    } else {
                                        None
                                    }
                            );
                        let right = 
                            down_right(hex).and_then(|t|
                                    if board.hexes[t].is_none()
                                    || has_colored_piece(board.hexes, t, color.opposite()) {
                                        Some(t)
                                    } else {
                                        None
                                    }
                            );
                            let dl = if let Some(t) = down_left(hex) {
                                board.hexes[t].is_none()
                            } else { false };
                            let dr = if let Some(t) = down_right(hex) {
                                board.hexes[t].is_none()
                            } else { false };
                            let d: Option<usize> = down_right(hex).and_then(|t| down_left(t));
                            let down =
                                if dl && dr {
                                    if let Some(d) = d {
                                        if board.hexes[d].is_none() {
                                            Some(d)
                                        } else { None }
                                    } else { None }
                                } else { None };
                        [left, right, down].iter().filter_map(|h| *h).collect()
                    }
                }
            }
            
        },
        None => {
            vec![]
        }
    }
}

pub fn get_king_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    let mut out = vec![];
    if let Some((color, _)) = hexes[hex] {
        for n in adjacent(hex) {
            if hexes[n].is_none() || has_colored_piece(hexes, n, color.opposite()) {
                out.push(n);
            }
        }
    }
    out
}

// pub fn get_fortress_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
//     let mut out = vec![];
//     for n in adjacent(hex) {
//         if hexes[n].is_none() {
//             out.push(n);
//         }
//     }
//     out
// }

pub fn get_knight_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    if let Some((color, _)) = hexes[hex] {
        let knight_moves = 
        [   
            [left, up_left],
            [up_left, up_left],
            [up_right, up_left],
            [up_right, up_right],
            [up_right, right],
            [right, right],
            [right, down_right],
            [down_right, down_right],
            [down_right, down_left],
            [down_left, down_left],
            [down_left, left],
            [left, left]
        ];
        knight_moves.iter().filter_map(
            |[f, g]| {
                if let Some(t) = f(hex) {
                    if let Some(h) = g(t) {
                        if hexes[h].is_none() || has_colored_piece(hexes, h, color.opposite()) {                
                            Some(h)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        ).collect()
    } else {
        vec![]
    }
}

pub fn get_queen_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    let mut out = vec![];
    if let Some((color, _)) = hexes[hex] {
        let queen_move_dirs = [
            up_left,
            up_right,
            right,
            down_right,
            down_left,
            left
        ];
        for dir in queen_move_dirs.iter() {
            let mut c = hex;
            'inner: loop {
                let h = dir(c);
                if let Some(h) = h {
                    c = h;
                    if hexes[h].is_none() {                
                        out.push(h);
                    } else if has_colored_piece(hexes, h, color.opposite()) {
                        out.push(h);
                        break 'inner;
                    } else {
                        break 'inner;
                    }
                } else {
                    break 'inner;
                }                
            }
        }
    }
    out
}

pub fn get_bishop_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    let mut out = vec![];
    if let Some((color, _)) = hexes[hex] {
        let bish_move_dirs = [
            up_left,
            up_right,
            down_right,
            down_left,
         ];
        for dir in bish_move_dirs.iter() {
            let mut c = hex;
            'inner: loop {
                let h = dir(c);
                if let Some(h) = h {
                    c = h;
                    if hexes[h].is_none() {                
                        out.push(h);
                    } else if has_colored_piece(hexes, h, color.opposite()) {
                        out.push(h);
                        break 'inner;
                    } else {
                        break 'inner;
                    }
                } else {
                    break 'inner;
                }                
            }
        }
    }
    out
}

pub fn get_fortress_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    let mut out = get_king_moves(hexes, hex);
    if let Some((_color, _)) = hexes[hex] {
        for n in adjacent(hex) {
            if hexes[n].is_none() {
                out.push(n);
            }
        }
        let two_hex_moves = 
        [   
            [left, up_left],
            [up_left, up_left],
            [up_right, up_left],
            [up_right, up_right],
            [up_right, right],
            [right, right],
            [right, down_right],
            [down_right, down_right],
            [down_right, down_left],
            [down_left, down_left],
            [down_left, left],
            [left, left]
        ];
        for [f, g] in two_hex_moves.iter() {
            'inner: for [fst, snd] in [[f, g], [g, f]].iter() {
                if let Some(one) = fst(hex) {
                    if hexes[one].is_none() {
                        if let Some(two) = snd(one) {
                            if hexes[two].is_none() { 
                                out.push(two);
                                break 'inner;
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

pub fn get_general_moves(hexes: Hexes, hex: usize) -> Vec<usize> {
    let mut out = vec![];
    if let Some((color, _)) = hexes[hex] {
        let moves = [
            [up_left, left],
            [up_right, right],
            [down_right, right],
            [down_left, left]
        ];
        for [fst, snd] in moves.iter() {
            if let Some(one) = fst(hex) {
                if has_colored_piece(hexes, one, color.opposite()) {
                    out.push(one);
                } else if hexes[one].is_none() {
                    out.push(one);
                    let mut c = one;
                    'inner: loop {
                        let h = snd(c);
                        if let Some(h) = h {
                            c = h;
                            if hexes[h].is_none() {
                                out.push(h);
                            } else if has_colored_piece(hexes, h, color.opposite()) {
                                out.push(h);
                                break 'inner;
                            } else {
                                break 'inner;
                            }
                        } else {
                            break 'inner;
                        }
                    }

                }
            }
        }
    }
    out
}

pub fn count_threats(board: &Board) -> [u8; 91] {
    let mut out = [0; 91];
    for hex in 0..91 {
        for vm in find_valid_moves(board, hex) {
            if board.hexes[vm].is_some() {
                out[vm] += 1;
            }
        }
    }
    out
}
