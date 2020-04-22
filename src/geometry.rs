
pub const COS_PI_6: f32 = 0.866025;
pub const HEX_VERTICES: [[f32; 2]; 6] = [
    [0.0, -1.0],
    [COS_PI_6, -0.5],
    [COS_PI_6, 0.5],
    [0.0, 1.0],
    [-COS_PI_6, 0.5],
    [-COS_PI_6, -0.5],
];

pub fn create_board_positions() -> [[f32; 2]; 91] {
    let mut out = [[0.0, 0.0]; 91];
    let mut hex = 0;
    for (rank, &rank_len) in crate::board::RANK_LENGTHS.iter().enumerate() {
        let mut x = crate::board::OFFS[rank] * COS_PI_6;
        let y = rank as f32 * 1.5;
        for _file in 0..rank_len {
            out[hex] = [x, y];
            x += 2.0 * COS_PI_6;
            hex += 1;
        }
    }
    out
}