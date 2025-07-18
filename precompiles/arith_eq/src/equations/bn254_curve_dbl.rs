// code generated
//
// equation: 2*s*y1-3*x1*x1+p*q0-p*offset
//
// p: 0x30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47
// offset: 0x100000000000000000000000000000000000000000000000000000000000000000
// 2: 2
// 3: 3
// (p*offset): 0x30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD4700000000000000000000000000000000000000000000000000000000000000000
//
// chunks:16
// chunk_bits:16
// terms_by_clock: 2

pub struct Bn254CurveDbl {}

impl Bn254CurveDbl {
    #[allow(clippy::too_many_arguments)]
    pub fn calculate(
        icol: u8,
        x1: &[i64; 16],
        y1: &[i64; 16],
        s: &[i64; 16],
        q0: &[i64; 16],
    ) -> i64 {
        match icol {
            0 => 2 * s[0] * y1[0] - 3 * x1[0] * x1[0] + 0xFD47 * q0[0],
            1 => {
                2 * s[1] * y1[0] + 2 * s[0] * y1[1] - 3 * x1[1] * x1[0] - 3 * x1[0] * x1[1]
                    + 0xD87C * q0[0]
                    + 0xFD47 * q0[1]
            }
            2 => {
                2 * s[2] * y1[0] + 2 * s[1] * y1[1] + 2 * s[0] * y1[2]
                    - 3 * x1[2] * x1[0]
                    - 3 * x1[1] * x1[1]
                    - 3 * x1[0] * x1[2]
                    + 0x8C16 * q0[0]
                    + 0xD87C * q0[1]
                    + 0xFD47 * q0[2]
            }
            3 => {
                2 * s[3] * y1[0] + 2 * s[2] * y1[1] + 2 * s[1] * y1[2] + 2 * s[0] * y1[3]
                    - 3 * x1[3] * x1[0]
                    - 3 * x1[2] * x1[1]
                    - 3 * x1[1] * x1[2]
                    - 3 * x1[0] * x1[3]
                    + 0x3C20 * q0[0]
                    + 0x8C16 * q0[1]
                    + 0xD87C * q0[2]
                    + 0xFD47 * q0[3]
            }
            4 => {
                2 * s[4] * y1[0]
                    + 2 * s[3] * y1[1]
                    + 2 * s[2] * y1[2]
                    + 2 * s[1] * y1[3]
                    + 2 * s[0] * y1[4]
                    - 3 * x1[4] * x1[0]
                    - 3 * x1[3] * x1[1]
                    - 3 * x1[2] * x1[2]
                    - 3 * x1[1] * x1[3]
                    - 3 * x1[0] * x1[4]
                    + 0xCA8D * q0[0]
                    + 0x3C20 * q0[1]
                    + 0x8C16 * q0[2]
                    + 0xD87C * q0[3]
                    + 0xFD47 * q0[4]
            }
            5 => {
                2 * s[5] * y1[0]
                    + 2 * s[4] * y1[1]
                    + 2 * s[3] * y1[2]
                    + 2 * s[2] * y1[3]
                    + 2 * s[1] * y1[4]
                    + 2 * s[0] * y1[5]
                    - 3 * x1[5] * x1[0]
                    - 3 * x1[4] * x1[1]
                    - 3 * x1[3] * x1[2]
                    - 3 * x1[2] * x1[3]
                    - 3 * x1[1] * x1[4]
                    - 3 * x1[0] * x1[5]
                    + 0x6871 * q0[0]
                    + 0xCA8D * q0[1]
                    + 0x3C20 * q0[2]
                    + 0x8C16 * q0[3]
                    + 0xD87C * q0[4]
                    + 0xFD47 * q0[5]
            }
            6 => {
                2 * s[6] * y1[0]
                    + 2 * s[5] * y1[1]
                    + 2 * s[4] * y1[2]
                    + 2 * s[3] * y1[3]
                    + 2 * s[2] * y1[4]
                    + 2 * s[1] * y1[5]
                    + 2 * s[0] * y1[6]
                    - 3 * x1[6] * x1[0]
                    - 3 * x1[5] * x1[1]
                    - 3 * x1[4] * x1[2]
                    - 3 * x1[3] * x1[3]
                    - 3 * x1[2] * x1[4]
                    - 3 * x1[1] * x1[5]
                    - 3 * x1[0] * x1[6]
                    + 0x6A91 * q0[0]
                    + 0x6871 * q0[1]
                    + 0xCA8D * q0[2]
                    + 0x3C20 * q0[3]
                    + 0x8C16 * q0[4]
                    + 0xD87C * q0[5]
                    + 0xFD47 * q0[6]
            }
            7 => {
                2 * s[7] * y1[0]
                    + 2 * s[6] * y1[1]
                    + 2 * s[5] * y1[2]
                    + 2 * s[4] * y1[3]
                    + 2 * s[3] * y1[4]
                    + 2 * s[2] * y1[5]
                    + 2 * s[1] * y1[6]
                    + 2 * s[0] * y1[7]
                    - 3 * x1[7] * x1[0]
                    - 3 * x1[6] * x1[1]
                    - 3 * x1[5] * x1[2]
                    - 3 * x1[4] * x1[3]
                    - 3 * x1[3] * x1[4]
                    - 3 * x1[2] * x1[5]
                    - 3 * x1[1] * x1[6]
                    - 3 * x1[0] * x1[7]
                    + 0x9781 * q0[0]
                    + 0x6A91 * q0[1]
                    + 0x6871 * q0[2]
                    + 0xCA8D * q0[3]
                    + 0x3C20 * q0[4]
                    + 0x8C16 * q0[5]
                    + 0xD87C * q0[6]
                    + 0xFD47 * q0[7]
            }
            8 => {
                2 * s[8] * y1[0]
                    + 2 * s[7] * y1[1]
                    + 2 * s[6] * y1[2]
                    + 2 * s[5] * y1[3]
                    + 2 * s[4] * y1[4]
                    + 2 * s[3] * y1[5]
                    + 2 * s[2] * y1[6]
                    + 2 * s[1] * y1[7]
                    + 2 * s[0] * y1[8]
                    - 3 * x1[8] * x1[0]
                    - 3 * x1[7] * x1[1]
                    - 3 * x1[6] * x1[2]
                    - 3 * x1[5] * x1[3]
                    - 3 * x1[4] * x1[4]
                    - 3 * x1[3] * x1[5]
                    - 3 * x1[2] * x1[6]
                    - 3 * x1[1] * x1[7]
                    - 3 * x1[0] * x1[8]
                    + 0x585D * q0[0]
                    + 0x9781 * q0[1]
                    + 0x6A91 * q0[2]
                    + 0x6871 * q0[3]
                    + 0xCA8D * q0[4]
                    + 0x3C20 * q0[5]
                    + 0x8C16 * q0[6]
                    + 0xD87C * q0[7]
                    + 0xFD47 * q0[8]
            }
            9 => {
                2 * s[9] * y1[0]
                    + 2 * s[8] * y1[1]
                    + 2 * s[7] * y1[2]
                    + 2 * s[6] * y1[3]
                    + 2 * s[5] * y1[4]
                    + 2 * s[4] * y1[5]
                    + 2 * s[3] * y1[6]
                    + 2 * s[2] * y1[7]
                    + 2 * s[1] * y1[8]
                    + 2 * s[0] * y1[9]
                    - 3 * x1[9] * x1[0]
                    - 3 * x1[8] * x1[1]
                    - 3 * x1[7] * x1[2]
                    - 3 * x1[6] * x1[3]
                    - 3 * x1[5] * x1[4]
                    - 3 * x1[4] * x1[5]
                    - 3 * x1[3] * x1[6]
                    - 3 * x1[2] * x1[7]
                    - 3 * x1[1] * x1[8]
                    - 3 * x1[0] * x1[9]
                    + 0x8181 * q0[0]
                    + 0x585D * q0[1]
                    + 0x9781 * q0[2]
                    + 0x6A91 * q0[3]
                    + 0x6871 * q0[4]
                    + 0xCA8D * q0[5]
                    + 0x3C20 * q0[6]
                    + 0x8C16 * q0[7]
                    + 0xD87C * q0[8]
                    + 0xFD47 * q0[9]
            }
            10 => {
                2 * s[10] * y1[0]
                    + 2 * s[9] * y1[1]
                    + 2 * s[8] * y1[2]
                    + 2 * s[7] * y1[3]
                    + 2 * s[6] * y1[4]
                    + 2 * s[5] * y1[5]
                    + 2 * s[4] * y1[6]
                    + 2 * s[3] * y1[7]
                    + 2 * s[2] * y1[8]
                    + 2 * s[1] * y1[9]
                    + 2 * s[0] * y1[10]
                    - 3 * x1[10] * x1[0]
                    - 3 * x1[9] * x1[1]
                    - 3 * x1[8] * x1[2]
                    - 3 * x1[7] * x1[3]
                    - 3 * x1[6] * x1[4]
                    - 3 * x1[5] * x1[5]
                    - 3 * x1[4] * x1[6]
                    - 3 * x1[3] * x1[7]
                    - 3 * x1[2] * x1[8]
                    - 3 * x1[1] * x1[9]
                    - 3 * x1[0] * x1[10]
                    + 0x45B6 * q0[0]
                    + 0x8181 * q0[1]
                    + 0x585D * q0[2]
                    + 0x9781 * q0[3]
                    + 0x6A91 * q0[4]
                    + 0x6871 * q0[5]
                    + 0xCA8D * q0[6]
                    + 0x3C20 * q0[7]
                    + 0x8C16 * q0[8]
                    + 0xD87C * q0[9]
                    + 0xFD47 * q0[10]
            }
            11 => {
                2 * s[11] * y1[0]
                    + 2 * s[10] * y1[1]
                    + 2 * s[9] * y1[2]
                    + 2 * s[8] * y1[3]
                    + 2 * s[7] * y1[4]
                    + 2 * s[6] * y1[5]
                    + 2 * s[5] * y1[6]
                    + 2 * s[4] * y1[7]
                    + 2 * s[3] * y1[8]
                    + 2 * s[2] * y1[9]
                    + 2 * s[1] * y1[10]
                    + 2 * s[0] * y1[11]
                    - 3 * x1[11] * x1[0]
                    - 3 * x1[10] * x1[1]
                    - 3 * x1[9] * x1[2]
                    - 3 * x1[8] * x1[3]
                    - 3 * x1[7] * x1[4]
                    - 3 * x1[6] * x1[5]
                    - 3 * x1[5] * x1[6]
                    - 3 * x1[4] * x1[7]
                    - 3 * x1[3] * x1[8]
                    - 3 * x1[2] * x1[9]
                    - 3 * x1[1] * x1[10]
                    - 3 * x1[0] * x1[11]
                    + 0xB850 * q0[0]
                    + 0x45B6 * q0[1]
                    + 0x8181 * q0[2]
                    + 0x585D * q0[3]
                    + 0x9781 * q0[4]
                    + 0x6A91 * q0[5]
                    + 0x6871 * q0[6]
                    + 0xCA8D * q0[7]
                    + 0x3C20 * q0[8]
                    + 0x8C16 * q0[9]
                    + 0xD87C * q0[10]
                    + 0xFD47 * q0[11]
            }
            12 => {
                2 * s[12] * y1[0]
                    + 2 * s[11] * y1[1]
                    + 2 * s[10] * y1[2]
                    + 2 * s[9] * y1[3]
                    + 2 * s[8] * y1[4]
                    + 2 * s[7] * y1[5]
                    + 2 * s[6] * y1[6]
                    + 2 * s[5] * y1[7]
                    + 2 * s[4] * y1[8]
                    + 2 * s[3] * y1[9]
                    + 2 * s[2] * y1[10]
                    + 2 * s[1] * y1[11]
                    + 2 * s[0] * y1[12]
                    - 3 * x1[12] * x1[0]
                    - 3 * x1[11] * x1[1]
                    - 3 * x1[10] * x1[2]
                    - 3 * x1[9] * x1[3]
                    - 3 * x1[8] * x1[4]
                    - 3 * x1[7] * x1[5]
                    - 3 * x1[6] * x1[6]
                    - 3 * x1[5] * x1[7]
                    - 3 * x1[4] * x1[8]
                    - 3 * x1[3] * x1[9]
                    - 3 * x1[2] * x1[10]
                    - 3 * x1[1] * x1[11]
                    - 3 * x1[0] * x1[12]
                    + 0xA029 * q0[0]
                    + 0xB850 * q0[1]
                    + 0x45B6 * q0[2]
                    + 0x8181 * q0[3]
                    + 0x585D * q0[4]
                    + 0x9781 * q0[5]
                    + 0x6A91 * q0[6]
                    + 0x6871 * q0[7]
                    + 0xCA8D * q0[8]
                    + 0x3C20 * q0[9]
                    + 0x8C16 * q0[10]
                    + 0xD87C * q0[11]
                    + 0xFD47 * q0[12]
            }
            13 => {
                2 * s[13] * y1[0]
                    + 2 * s[12] * y1[1]
                    + 2 * s[11] * y1[2]
                    + 2 * s[10] * y1[3]
                    + 2 * s[9] * y1[4]
                    + 2 * s[8] * y1[5]
                    + 2 * s[7] * y1[6]
                    + 2 * s[6] * y1[7]
                    + 2 * s[5] * y1[8]
                    + 2 * s[4] * y1[9]
                    + 2 * s[3] * y1[10]
                    + 2 * s[2] * y1[11]
                    + 2 * s[1] * y1[12]
                    + 2 * s[0] * y1[13]
                    - 3 * x1[13] * x1[0]
                    - 3 * x1[12] * x1[1]
                    - 3 * x1[11] * x1[2]
                    - 3 * x1[10] * x1[3]
                    - 3 * x1[9] * x1[4]
                    - 3 * x1[8] * x1[5]
                    - 3 * x1[7] * x1[6]
                    - 3 * x1[6] * x1[7]
                    - 3 * x1[5] * x1[8]
                    - 3 * x1[4] * x1[9]
                    - 3 * x1[3] * x1[10]
                    - 3 * x1[2] * x1[11]
                    - 3 * x1[1] * x1[12]
                    - 3 * x1[0] * x1[13]
                    + 0xE131 * q0[0]
                    + 0xA029 * q0[1]
                    + 0xB850 * q0[2]
                    + 0x45B6 * q0[3]
                    + 0x8181 * q0[4]
                    + 0x585D * q0[5]
                    + 0x9781 * q0[6]
                    + 0x6A91 * q0[7]
                    + 0x6871 * q0[8]
                    + 0xCA8D * q0[9]
                    + 0x3C20 * q0[10]
                    + 0x8C16 * q0[11]
                    + 0xD87C * q0[12]
                    + 0xFD47 * q0[13]
            }
            14 => {
                2 * s[14] * y1[0]
                    + 2 * s[13] * y1[1]
                    + 2 * s[12] * y1[2]
                    + 2 * s[11] * y1[3]
                    + 2 * s[10] * y1[4]
                    + 2 * s[9] * y1[5]
                    + 2 * s[8] * y1[6]
                    + 2 * s[7] * y1[7]
                    + 2 * s[6] * y1[8]
                    + 2 * s[5] * y1[9]
                    + 2 * s[4] * y1[10]
                    + 2 * s[3] * y1[11]
                    + 2 * s[2] * y1[12]
                    + 2 * s[1] * y1[13]
                    + 2 * s[0] * y1[14]
                    - 3 * x1[14] * x1[0]
                    - 3 * x1[13] * x1[1]
                    - 3 * x1[12] * x1[2]
                    - 3 * x1[11] * x1[3]
                    - 3 * x1[10] * x1[4]
                    - 3 * x1[9] * x1[5]
                    - 3 * x1[8] * x1[6]
                    - 3 * x1[7] * x1[7]
                    - 3 * x1[6] * x1[8]
                    - 3 * x1[5] * x1[9]
                    - 3 * x1[4] * x1[10]
                    - 3 * x1[3] * x1[11]
                    - 3 * x1[2] * x1[12]
                    - 3 * x1[1] * x1[13]
                    - 3 * x1[0] * x1[14]
                    + 0x4E72 * q0[0]
                    + 0xE131 * q0[1]
                    + 0xA029 * q0[2]
                    + 0xB850 * q0[3]
                    + 0x45B6 * q0[4]
                    + 0x8181 * q0[5]
                    + 0x585D * q0[6]
                    + 0x9781 * q0[7]
                    + 0x6A91 * q0[8]
                    + 0x6871 * q0[9]
                    + 0xCA8D * q0[10]
                    + 0x3C20 * q0[11]
                    + 0x8C16 * q0[12]
                    + 0xD87C * q0[13]
                    + 0xFD47 * q0[14]
            }
            15 => {
                2 * s[15] * y1[0]
                    + 2 * s[14] * y1[1]
                    + 2 * s[13] * y1[2]
                    + 2 * s[12] * y1[3]
                    + 2 * s[11] * y1[4]
                    + 2 * s[10] * y1[5]
                    + 2 * s[9] * y1[6]
                    + 2 * s[8] * y1[7]
                    + 2 * s[7] * y1[8]
                    + 2 * s[6] * y1[9]
                    + 2 * s[5] * y1[10]
                    + 2 * s[4] * y1[11]
                    + 2 * s[3] * y1[12]
                    + 2 * s[2] * y1[13]
                    + 2 * s[1] * y1[14]
                    + 2 * s[0] * y1[15]
                    - 3 * x1[15] * x1[0]
                    - 3 * x1[14] * x1[1]
                    - 3 * x1[13] * x1[2]
                    - 3 * x1[12] * x1[3]
                    - 3 * x1[11] * x1[4]
                    - 3 * x1[10] * x1[5]
                    - 3 * x1[9] * x1[6]
                    - 3 * x1[8] * x1[7]
                    - 3 * x1[7] * x1[8]
                    - 3 * x1[6] * x1[9]
                    - 3 * x1[5] * x1[10]
                    - 3 * x1[4] * x1[11]
                    - 3 * x1[3] * x1[12]
                    - 3 * x1[2] * x1[13]
                    - 3 * x1[1] * x1[14]
                    - 3 * x1[0] * x1[15]
                    + 0x3064 * q0[0]
                    + 0x4E72 * q0[1]
                    + 0xE131 * q0[2]
                    + 0xA029 * q0[3]
                    + 0xB850 * q0[4]
                    + 0x45B6 * q0[5]
                    + 0x8181 * q0[6]
                    + 0x585D * q0[7]
                    + 0x9781 * q0[8]
                    + 0x6A91 * q0[9]
                    + 0x6871 * q0[10]
                    + 0xCA8D * q0[11]
                    + 0x3C20 * q0[12]
                    + 0x8C16 * q0[13]
                    + 0xD87C * q0[14]
                    + 0xFD47 * q0[15]
            }
            16 => {
                2 * s[15] * y1[1]
                    + 2 * s[14] * y1[2]
                    + 2 * s[13] * y1[3]
                    + 2 * s[12] * y1[4]
                    + 2 * s[11] * y1[5]
                    + 2 * s[10] * y1[6]
                    + 2 * s[9] * y1[7]
                    + 2 * s[8] * y1[8]
                    + 2 * s[7] * y1[9]
                    + 2 * s[6] * y1[10]
                    + 2 * s[5] * y1[11]
                    + 2 * s[4] * y1[12]
                    + 2 * s[3] * y1[13]
                    + 2 * s[2] * y1[14]
                    + 2 * s[1] * y1[15]
                    - 3 * x1[15] * x1[1]
                    - 3 * x1[14] * x1[2]
                    - 3 * x1[13] * x1[3]
                    - 3 * x1[12] * x1[4]
                    - 3 * x1[11] * x1[5]
                    - 3 * x1[10] * x1[6]
                    - 3 * x1[9] * x1[7]
                    - 3 * x1[8] * x1[8]
                    - 3 * x1[7] * x1[9]
                    - 3 * x1[6] * x1[10]
                    - 3 * x1[5] * x1[11]
                    - 3 * x1[4] * x1[12]
                    - 3 * x1[3] * x1[13]
                    - 3 * x1[2] * x1[14]
                    - 3 * x1[1] * x1[15]
                    + 0x3064 * q0[1]
                    + 0x4E72 * q0[2]
                    + 0xE131 * q0[3]
                    + 0xA029 * q0[4]
                    + 0xB850 * q0[5]
                    + 0x45B6 * q0[6]
                    + 0x8181 * q0[7]
                    + 0x585D * q0[8]
                    + 0x9781 * q0[9]
                    + 0x6A91 * q0[10]
                    + 0x6871 * q0[11]
                    + 0xCA8D * q0[12]
                    + 0x3C20 * q0[13]
                    + 0x8C16 * q0[14]
                    + 0xD87C * q0[15]
                    - 0xD470
            }
            17 => {
                2 * s[15] * y1[2]
                    + 2 * s[14] * y1[3]
                    + 2 * s[13] * y1[4]
                    + 2 * s[12] * y1[5]
                    + 2 * s[11] * y1[6]
                    + 2 * s[10] * y1[7]
                    + 2 * s[9] * y1[8]
                    + 2 * s[8] * y1[9]
                    + 2 * s[7] * y1[10]
                    + 2 * s[6] * y1[11]
                    + 2 * s[5] * y1[12]
                    + 2 * s[4] * y1[13]
                    + 2 * s[3] * y1[14]
                    + 2 * s[2] * y1[15]
                    - 3 * x1[15] * x1[2]
                    - 3 * x1[14] * x1[3]
                    - 3 * x1[13] * x1[4]
                    - 3 * x1[12] * x1[5]
                    - 3 * x1[11] * x1[6]
                    - 3 * x1[10] * x1[7]
                    - 3 * x1[9] * x1[8]
                    - 3 * x1[8] * x1[9]
                    - 3 * x1[7] * x1[10]
                    - 3 * x1[6] * x1[11]
                    - 3 * x1[5] * x1[12]
                    - 3 * x1[4] * x1[13]
                    - 3 * x1[3] * x1[14]
                    - 3 * x1[2] * x1[15]
                    + 0x3064 * q0[2]
                    + 0x4E72 * q0[3]
                    + 0xE131 * q0[4]
                    + 0xA029 * q0[5]
                    + 0xB850 * q0[6]
                    + 0x45B6 * q0[7]
                    + 0x8181 * q0[8]
                    + 0x585D * q0[9]
                    + 0x9781 * q0[10]
                    + 0x6A91 * q0[11]
                    + 0x6871 * q0[12]
                    + 0xCA8D * q0[13]
                    + 0x3C20 * q0[14]
                    + 0x8C16 * q0[15]
                    - 0x87CF
            }
            18 => {
                2 * s[15] * y1[3]
                    + 2 * s[14] * y1[4]
                    + 2 * s[13] * y1[5]
                    + 2 * s[12] * y1[6]
                    + 2 * s[11] * y1[7]
                    + 2 * s[10] * y1[8]
                    + 2 * s[9] * y1[9]
                    + 2 * s[8] * y1[10]
                    + 2 * s[7] * y1[11]
                    + 2 * s[6] * y1[12]
                    + 2 * s[5] * y1[13]
                    + 2 * s[4] * y1[14]
                    + 2 * s[3] * y1[15]
                    - 3 * x1[15] * x1[3]
                    - 3 * x1[14] * x1[4]
                    - 3 * x1[13] * x1[5]
                    - 3 * x1[12] * x1[6]
                    - 3 * x1[11] * x1[7]
                    - 3 * x1[10] * x1[8]
                    - 3 * x1[9] * x1[9]
                    - 3 * x1[8] * x1[10]
                    - 3 * x1[7] * x1[11]
                    - 3 * x1[6] * x1[12]
                    - 3 * x1[5] * x1[13]
                    - 3 * x1[4] * x1[14]
                    - 3 * x1[3] * x1[15]
                    + 0x3064 * q0[3]
                    + 0x4E72 * q0[4]
                    + 0xE131 * q0[5]
                    + 0xA029 * q0[6]
                    + 0xB850 * q0[7]
                    + 0x45B6 * q0[8]
                    + 0x8181 * q0[9]
                    + 0x585D * q0[10]
                    + 0x9781 * q0[11]
                    + 0x6A91 * q0[12]
                    + 0x6871 * q0[13]
                    + 0xCA8D * q0[14]
                    + 0x3C20 * q0[15]
                    - 0xC16D
            }
            19 => {
                2 * s[15] * y1[4]
                    + 2 * s[14] * y1[5]
                    + 2 * s[13] * y1[6]
                    + 2 * s[12] * y1[7]
                    + 2 * s[11] * y1[8]
                    + 2 * s[10] * y1[9]
                    + 2 * s[9] * y1[10]
                    + 2 * s[8] * y1[11]
                    + 2 * s[7] * y1[12]
                    + 2 * s[6] * y1[13]
                    + 2 * s[5] * y1[14]
                    + 2 * s[4] * y1[15]
                    - 3 * x1[15] * x1[4]
                    - 3 * x1[14] * x1[5]
                    - 3 * x1[13] * x1[6]
                    - 3 * x1[12] * x1[7]
                    - 3 * x1[11] * x1[8]
                    - 3 * x1[10] * x1[9]
                    - 3 * x1[9] * x1[10]
                    - 3 * x1[8] * x1[11]
                    - 3 * x1[7] * x1[12]
                    - 3 * x1[6] * x1[13]
                    - 3 * x1[5] * x1[14]
                    - 3 * x1[4] * x1[15]
                    + 0x3064 * q0[4]
                    + 0x4E72 * q0[5]
                    + 0xE131 * q0[6]
                    + 0xA029 * q0[7]
                    + 0xB850 * q0[8]
                    + 0x45B6 * q0[9]
                    + 0x8181 * q0[10]
                    + 0x585D * q0[11]
                    + 0x9781 * q0[12]
                    + 0x6A91 * q0[13]
                    + 0x6871 * q0[14]
                    + 0xCA8D * q0[15]
                    - 0xC208
            }
            20 => {
                2 * s[15] * y1[5]
                    + 2 * s[14] * y1[6]
                    + 2 * s[13] * y1[7]
                    + 2 * s[12] * y1[8]
                    + 2 * s[11] * y1[9]
                    + 2 * s[10] * y1[10]
                    + 2 * s[9] * y1[11]
                    + 2 * s[8] * y1[12]
                    + 2 * s[7] * y1[13]
                    + 2 * s[6] * y1[14]
                    + 2 * s[5] * y1[15]
                    - 3 * x1[15] * x1[5]
                    - 3 * x1[14] * x1[6]
                    - 3 * x1[13] * x1[7]
                    - 3 * x1[12] * x1[8]
                    - 3 * x1[11] * x1[9]
                    - 3 * x1[10] * x1[10]
                    - 3 * x1[9] * x1[11]
                    - 3 * x1[8] * x1[12]
                    - 3 * x1[7] * x1[13]
                    - 3 * x1[6] * x1[14]
                    - 3 * x1[5] * x1[15]
                    + 0x3064 * q0[5]
                    + 0x4E72 * q0[6]
                    + 0xE131 * q0[7]
                    + 0xA029 * q0[8]
                    + 0xB850 * q0[9]
                    + 0x45B6 * q0[10]
                    + 0x8181 * q0[11]
                    + 0x585D * q0[12]
                    + 0x9781 * q0[13]
                    + 0x6A91 * q0[14]
                    + 0x6871 * q0[15]
                    - 0xA8D3
            }
            21 => {
                2 * s[15] * y1[6]
                    + 2 * s[14] * y1[7]
                    + 2 * s[13] * y1[8]
                    + 2 * s[12] * y1[9]
                    + 2 * s[11] * y1[10]
                    + 2 * s[10] * y1[11]
                    + 2 * s[9] * y1[12]
                    + 2 * s[8] * y1[13]
                    + 2 * s[7] * y1[14]
                    + 2 * s[6] * y1[15]
                    - 3 * x1[15] * x1[6]
                    - 3 * x1[14] * x1[7]
                    - 3 * x1[13] * x1[8]
                    - 3 * x1[12] * x1[9]
                    - 3 * x1[11] * x1[10]
                    - 3 * x1[10] * x1[11]
                    - 3 * x1[9] * x1[12]
                    - 3 * x1[8] * x1[13]
                    - 3 * x1[7] * x1[14]
                    - 3 * x1[6] * x1[15]
                    + 0x3064 * q0[6]
                    + 0x4E72 * q0[7]
                    + 0xE131 * q0[8]
                    + 0xA029 * q0[9]
                    + 0xB850 * q0[10]
                    + 0x45B6 * q0[11]
                    + 0x8181 * q0[12]
                    + 0x585D * q0[13]
                    + 0x9781 * q0[14]
                    + 0x6A91 * q0[15]
                    - 0x871C
            }
            22 => {
                2 * s[15] * y1[7]
                    + 2 * s[14] * y1[8]
                    + 2 * s[13] * y1[9]
                    + 2 * s[12] * y1[10]
                    + 2 * s[11] * y1[11]
                    + 2 * s[10] * y1[12]
                    + 2 * s[9] * y1[13]
                    + 2 * s[8] * y1[14]
                    + 2 * s[7] * y1[15]
                    - 3 * x1[15] * x1[7]
                    - 3 * x1[14] * x1[8]
                    - 3 * x1[13] * x1[9]
                    - 3 * x1[12] * x1[10]
                    - 3 * x1[11] * x1[11]
                    - 3 * x1[10] * x1[12]
                    - 3 * x1[9] * x1[13]
                    - 3 * x1[8] * x1[14]
                    - 3 * x1[7] * x1[15]
                    + 0x3064 * q0[7]
                    + 0x4E72 * q0[8]
                    + 0xE131 * q0[9]
                    + 0xA029 * q0[10]
                    + 0xB850 * q0[11]
                    + 0x45B6 * q0[12]
                    + 0x8181 * q0[13]
                    + 0x585D * q0[14]
                    + 0x9781 * q0[15]
                    - 0xA916
            }
            23 => {
                2 * s[15] * y1[8]
                    + 2 * s[14] * y1[9]
                    + 2 * s[13] * y1[10]
                    + 2 * s[12] * y1[11]
                    + 2 * s[11] * y1[12]
                    + 2 * s[10] * y1[13]
                    + 2 * s[9] * y1[14]
                    + 2 * s[8] * y1[15]
                    - 3 * x1[15] * x1[8]
                    - 3 * x1[14] * x1[9]
                    - 3 * x1[13] * x1[10]
                    - 3 * x1[12] * x1[11]
                    - 3 * x1[11] * x1[12]
                    - 3 * x1[10] * x1[13]
                    - 3 * x1[9] * x1[14]
                    - 3 * x1[8] * x1[15]
                    + 0x3064 * q0[8]
                    + 0x4E72 * q0[9]
                    + 0xE131 * q0[10]
                    + 0xA029 * q0[11]
                    + 0xB850 * q0[12]
                    + 0x45B6 * q0[13]
                    + 0x8181 * q0[14]
                    + 0x585D * q0[15]
                    - 0x7816
            }
            24 => {
                2 * s[15] * y1[9]
                    + 2 * s[14] * y1[10]
                    + 2 * s[13] * y1[11]
                    + 2 * s[12] * y1[12]
                    + 2 * s[11] * y1[13]
                    + 2 * s[10] * y1[14]
                    + 2 * s[9] * y1[15]
                    - 3 * x1[15] * x1[9]
                    - 3 * x1[14] * x1[10]
                    - 3 * x1[13] * x1[11]
                    - 3 * x1[12] * x1[12]
                    - 3 * x1[11] * x1[13]
                    - 3 * x1[10] * x1[14]
                    - 3 * x1[9] * x1[15]
                    + 0x3064 * q0[9]
                    + 0x4E72 * q0[10]
                    + 0xE131 * q0[11]
                    + 0xA029 * q0[12]
                    + 0xB850 * q0[13]
                    + 0x45B6 * q0[14]
                    + 0x8181 * q0[15]
                    - 0x85D9
            }
            25 => {
                2 * s[15] * y1[10]
                    + 2 * s[14] * y1[11]
                    + 2 * s[13] * y1[12]
                    + 2 * s[12] * y1[13]
                    + 2 * s[11] * y1[14]
                    + 2 * s[10] * y1[15]
                    - 3 * x1[15] * x1[10]
                    - 3 * x1[14] * x1[11]
                    - 3 * x1[13] * x1[12]
                    - 3 * x1[12] * x1[13]
                    - 3 * x1[11] * x1[14]
                    - 3 * x1[10] * x1[15]
                    + 0x3064 * q0[10]
                    + 0x4E72 * q0[11]
                    + 0xE131 * q0[12]
                    + 0xA029 * q0[13]
                    + 0xB850 * q0[14]
                    + 0x45B6 * q0[15]
                    - 0x1815
            }
            26 => {
                2 * s[15] * y1[11]
                    + 2 * s[14] * y1[12]
                    + 2 * s[13] * y1[13]
                    + 2 * s[12] * y1[14]
                    + 2 * s[11] * y1[15]
                    - 3 * x1[15] * x1[11]
                    - 3 * x1[14] * x1[12]
                    - 3 * x1[13] * x1[13]
                    - 3 * x1[12] * x1[14]
                    - 3 * x1[11] * x1[15]
                    + 0x3064 * q0[11]
                    + 0x4E72 * q0[12]
                    + 0xE131 * q0[13]
                    + 0xA029 * q0[14]
                    + 0xB850 * q0[15]
                    - 0x5B68
            }
            27 => {
                2 * s[15] * y1[12] + 2 * s[14] * y1[13] + 2 * s[13] * y1[14] + 2 * s[12] * y1[15]
                    - 3 * x1[15] * x1[12]
                    - 3 * x1[14] * x1[13]
                    - 3 * x1[13] * x1[14]
                    - 3 * x1[12] * x1[15]
                    + 0x3064 * q0[12]
                    + 0x4E72 * q0[13]
                    + 0xE131 * q0[14]
                    + 0xA029 * q0[15]
                    - 0x8504
            }
            28 => {
                2 * s[15] * y1[13] + 2 * s[14] * y1[14] + 2 * s[13] * y1[15]
                    - 3 * x1[15] * x1[13]
                    - 3 * x1[14] * x1[14]
                    - 3 * x1[13] * x1[15]
                    + 0x3064 * q0[13]
                    + 0x4E72 * q0[14]
                    + 0xE131 * q0[15]
                    - 0x29B
            }
            29 => {
                2 * s[15] * y1[14] + 2 * s[14] * y1[15] - 3 * x1[15] * x1[14] - 3 * x1[14] * x1[15]
                    + 0x3064 * q0[14]
                    + 0x4E72 * q0[15]
                    - 0x131A
            }
            30 => 2 * s[15] * y1[15] - 3 * x1[15] * x1[15] + 0x3064 * q0[15] - 0xE72E,
            31 => -0x30644,
            _ => 0,
        }
    }
}
