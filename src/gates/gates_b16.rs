// Create a library of Logical 16-bit Binary Gates, such as AND, OR, NOT, XOR, etc.
// input must be fixed size binary array that has 16 elements.

mod gates_b16 {
    // AND Gate
    pub fn and(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [a[0] && b[0], a[1] && b[1], a[2] && b[2], a[3] && b[3],
         a[4] && b[4], a[5] && b[5], a[6] && b[6], a[7] && b[7],
         a[8] && b[8], a[9] && b[9], a[10] && b[10], a[11] && b[11],
         a[12] && b[12], a[13] && b[13], a[14] && b[14], a[15] && b[15]]
    }

    // OR Gate
    pub fn or(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [a[0] || b[0], a[1] || b[1], a[2] || b[2], a[3] || b[3],
         a[4] || b[4], a[5] || b[5], a[6] || b[6], a[7] || b[7],
         a[8] || b[8], a[9] || b[9], a[10] || b[10], a[11] || b[11],
         a[12] || b[12], a[13] || b[13], a[14] || b[14], a[15] || b[15]]
    }

    // NOT Gate
    pub fn not(a: [bool; 16]) -> [bool; 16] {
        [!a[0], !a[1], !a[2], !a[3], !a[4], !a[5], !a[6], !a[7],
         !a[8], !a[9], !a[10], !a[11], !a[12], !a[13], !a[14], !a[15]]
    }

    // XOR Gate
    pub fn xor(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3],
         a[4] ^ b[4], a[5] ^ b[5], a[6] ^ b[6], a[7] ^ b[7],
         a[8] ^ b[8], a[9] ^ b[9], a[10] ^ b[10], a[11] ^ b[11],
         a[12] ^ b[12], a[13] ^ b[13], a[14] ^ b[14], a[15] ^ b[15]]
    }

    // NAND Gate
    pub fn nand(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [!(a[0] && b[0]), !(a[1] && b[1]), !(a[2] && b[2]), !(a[3] && b[3]),
         !(a[4] && b[4]), !(a[5] && b[5]), !(a[6] && b[6]), !(a[7] && b[7]),
         !(a[8] && b[8]), !(a[9] && b[9]), !(a[10] && b[10]), !(a[11] && b[11]),
         !(a[12] && b[12]), !(a[13] && b[13]), !(a[14] && b[14]), !(a[15] && b[15])]
    }

    // NOR Gate
    pub fn nor(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [!(a[0] || b[0]), !(a[1] || b[1]), !(a[2] || b[2]), !(a[3] || b[3]),
         !(a[4] || b[4]), !(a[5] || b[5]), !(a[6] || b[6]), !(a[7] || b[7]),
         !(a[8] || b[8]), !(a[9] || b[9]), !(a[10] || b[10]), !(a[11] || b[11]),
         !(a[12] || b[12]), !(a[13] || b[13]), !(a[14] || b[14]), !(a[15] || b[15])]
    }

    // XNOR Gate
    pub fn xnor(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
        [!(a[0] ^ b[0]), !(a[1] ^ b[1]), !(a[2] ^ b[2]), !(a[3] ^ b[3]),
         !(a[4] ^ b[4]), !(a[5] ^ b[5]), !(a[6] ^ b[6]), !(a[7] ^ b[7]),
         !(a[8] ^ b[8]), !(a[9] ^ b[9]), !(a[10] ^ b[10]), !(a[11] ^ b[11]),
         !(a[12] ^ b[12]), !(a[13] ^ b[13]), !(a[14] ^ b[14]), !(a[15] ^ b[15])]
    }

    // MUX
    pub fn mux(a: [bool; 16], b: [bool; 16], sel: bool) -> [bool; 16] {
        [
            (a[0] && !sel) || (b[0] && sel),
            (a[1] && !sel) || (b[1] && sel),
            (a[2] && !sel) || (b[2] && sel),
            (a[3] && !sel) || (b[3] && sel),
            (a[4] && !sel) || (b[4] && sel),
            (a[5] && !sel) || (b[5] && sel),
            (a[6] && !sel) || (b[6] && sel),
            (a[7] && !sel) || (b[7] && sel),
            (a[8] && !sel) || (b[8] && sel),
            (a[9] && !sel) || (b[9] && sel),
            (a[10] && !sel) || (b[10] && sel),
            (a[11] && !sel) || (b[11] && sel),
            (a[12] && !sel) || (b[12] && sel),
            (a[13] && !sel) || (b[13] && sel),
            (a[14] && !sel) || (b[14] && sel),
            (a[15] && !sel) || (b[15] && sel),
        ]
    }

    // DMUX
    pub fn dmux(a: [bool; 16], sel: bool) -> ([bool; 16], [bool; 16]) {
        (
            [
                a[0] && !sel,
                a[1] && !sel,
                a[2] && !sel,
                a[3] && !sel,
                a[4] && !sel,
                a[5] && !sel,
                a[6] && !sel,
                a[7] && !sel,
                a[8] && !sel,
                a[9] && !sel,
                a[10] && !sel,
                a[11] && !sel,
                a[12] && !sel,
                a[13] && !sel,
                a[14] && !sel,
                a[15] && !sel,
            ],
            [
                a[0] && sel,
                a[1] && sel,
                a[2] && sel,
                a[3] && sel,
                a[4] && sel,
                a[5] && sel,
                a[6] && sel,
                a[7] && sel,
                a[8] && sel,
                a[9] && sel,
                a[10] && sel,
                a[11] && sel,
                a[12] && sel,
                a[13] && sel,
                a[14] && sel,
                a[15] && sel,
            ],
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const B16_TRUE: [bool; 16] = [  true,true,true,true,
                                    true,true,true,true,
                                    true,true,true,true,
                                    true,true,true,true];
    const B16_FALSE: [bool; 16] = [ false,false,false,false,
                                    false,false,false,false,
                                    false,false,false,false,
                                    false,false,false,false];

    #[test]
    fn test_and() {
        assert_eq!(gates_b16::and(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(gates_b16::and(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(gates_b16::and(B16_FALSE, B16_TRUE), B16_FALSE);                 
        assert_eq!(gates_b16::and(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_or() {
        assert_eq!(gates_b16::or(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(gates_b16::or(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(gates_b16::or(B16_FALSE, B16_TRUE), B16_TRUE);                 
        assert_eq!(gates_b16::or(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_not() {
        assert_eq!(gates_b16::not(B16_TRUE), B16_FALSE);
        assert_eq!(gates_b16::not(B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_xor() {
        assert_eq!(gates_b16::xor(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(gates_b16::xor(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(gates_b16::xor(B16_FALSE, B16_TRUE), B16_TRUE);                 
        assert_eq!(gates_b16::xor(B16_FALSE, B16_FALSE), B16_FALSE);
    }

    #[test]
    fn test_nand() {
        assert_eq!(gates_b16::nand(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(gates_b16::nand(B16_TRUE, B16_FALSE), B16_TRUE);
        assert_eq!(gates_b16::nand(B16_FALSE, B16_TRUE), B16_TRUE);                 
        assert_eq!(gates_b16::nand(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_nor() {
        assert_eq!(gates_b16::nor(B16_TRUE, B16_TRUE), B16_FALSE);
        assert_eq!(gates_b16::nor(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(gates_b16::nor(B16_FALSE, B16_TRUE), B16_FALSE);                 
        assert_eq!(gates_b16::nor(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(gates_b16::xnor(B16_TRUE, B16_TRUE), B16_TRUE);
        assert_eq!(gates_b16::xnor(B16_TRUE, B16_FALSE), B16_FALSE);
        assert_eq!(gates_b16::xnor(B16_FALSE, B16_TRUE), B16_FALSE);                 
        assert_eq!(gates_b16::xnor(B16_FALSE, B16_FALSE), B16_TRUE);
    }

    #[test]
    fn test_mux() {
        assert_eq!(gates_b16::mux(B16_TRUE, B16_FALSE, false), B16_TRUE);
        assert_eq!(gates_b16::mux(B16_TRUE, B16_FALSE, true), B16_FALSE);
        assert_eq!(gates_b16::mux(B16_FALSE, B16_TRUE, false), B16_FALSE);                 
        assert_eq!(gates_b16::mux(B16_FALSE, B16_TRUE, true), B16_TRUE);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(gates_b16::dmux(B16_TRUE, false), (B16_TRUE, B16_FALSE));
        assert_eq!(gates_b16::dmux(B16_TRUE, true), (B16_FALSE, B16_TRUE));
        assert_eq!(gates_b16::dmux(B16_FALSE, false), (B16_FALSE, B16_FALSE));                 
        assert_eq!(gates_b16::dmux(B16_FALSE, true), (B16_FALSE, B16_FALSE));
    }
}
