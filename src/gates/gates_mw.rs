
// Create multi-way gates:
//   - Or8Way
//   - Mux4Way16
//   - Mux8Way16
//   - DMux4Way
//   - DMux8Way

use crate::gates::gates_b16::mux16;
use crate::gates::gates_b16::dmux16;

pub fn or8way(a: [bool; 8]) -> bool {
    a[0] || a[1] || a[2] || a[3] || a[4] || a[5] || a[6] || a[7]
}

pub fn mux4way16(
    a: [bool; 16], 
    b: [bool; 16], 
    c: [bool; 16],
    d: [bool; 16],
    s: [bool; 2]) -> [bool; 16]  {
    mux16(
        mux16(a, b, s[0]),
        mux16(c, d, s[0]),
        s[1])
}

pub fn mux8way16(
    a: [bool; 16], 
    b: [bool; 16], 
    c: [bool; 16],
    d: [bool; 16],
    e: [bool; 16], 
    f: [bool; 16], 
    g: [bool; 16],
    h: [bool; 16],
    s: [bool; 3]) -> [bool; 16]  {
    mux16(
    mux16(
        mux16(a, b, s[0]),
        mux16(c, d, s[0]),
        s[1]),
    mux16(
        mux16(e, f, s[0]),
        mux16(g, h, s[0]),
        s[1]),
    s[2])
}

pub fn dmux4way(
    input: [bool; 16],
    s: [bool; 2]) -> ([bool; 16], [bool; 16], [bool; 16], [bool; 16]) {
    let (ab, cd) = dmux16(input, s[1]);
    let (a, b) = dmux16(ab, s[0]);
    let (c, d) = dmux16(cd, s[0]);
    
    (a, b, c, d)
}

pub fn dmux8way(
    input: [bool; 16],
    s: [bool; 3]) -> (  [bool; 16], [bool; 16], 
                        [bool; 16], [bool; 16],
                        [bool; 16], [bool; 16], 
                        [bool; 16], [bool; 16]) {

    let (abcd, efgh) = dmux16(input, s[2]);
    let (ab, cd) = dmux16(abcd, s[1]);
    let (ef, gh) = dmux16(efgh, s[1]);


    let (a, b) = dmux16(ab, s[0]);
    let (c, d) = dmux16(cd, s[0]);
    let (e, f) = dmux16(ef, s[0]);
    let (g, h) = dmux16(gh, s[0]);

    (a, b, c, d, e, f, g, h)
}

// TODO: Add tests for the new gates
