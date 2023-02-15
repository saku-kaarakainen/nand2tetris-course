// Create a library of Logical 2-bit Binary Gates, such as AND, OR, NOT, XOR, etc.
// input must be fixed size binary array that has 2 elements.

mod two_bit_gates {
    // AND Gate
    pub fn and(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [a[0] && b[0], a[1] && b[1]]
    }

    // OR Gate
    pub fn or(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [a[0] || b[0], a[1] || b[1]]
    }

    // NOT Gate
    pub fn not(a: [bool; 2]) -> [bool; 2] {
        [!a[0], !a[1]]
    }

    // XOR Gate
    pub fn xor(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [a[0] ^ b[0], a[1] ^ b[1]]
    }

    // NAND Gate
    pub fn nand(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [!(a[0] && b[0]), !(a[1] && b[1])]
    }

    // NOR Gate
    pub fn nor(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [!(a[0] || b[0]), !(a[1] || b[1])]
    }

    // XNOR Gate
    pub fn xnor(a: [bool; 2], b: [bool; 2]) -> [bool; 2] {
        [!(a[0] ^ b[0]), !(a[1] ^ b[1])]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(two_bit_gates::and([true, true], [true, true]), [true, true]);
        assert_eq!(two_bit_gates::and([true, true], [false, false]), [false, false]);
        assert_eq!(two_bit_gates::and([true, false], [true, false]), [true, false]);
        assert_eq!(two_bit_gates::and([false, false], [false, false]), [false, false]);
    }

    #[test]
    fn test_or() {
        assert_eq!(two_bit_gates::or([true, true], [true, true]), [true, true]);
        assert_eq!(two_bit_gates::or([true, true], [false, false]), [true, true]);
        assert_eq!(two_bit_gates::or([true, false], [true, false]), [true, false]);
        assert_eq!(two_bit_gates::or([false, false], [false, false]), [false, false]);
    }

    #[test]
    fn test_not() {
        assert_eq!(two_bit_gates::not([true, true]), [false, false]);
        assert_eq!(two_bit_gates::not([true, false]), [false, true]);
        assert_eq!(two_bit_gates::not([false, true]), [true, false]);
        assert_eq!(two_bit_gates::not([false, false]), [true, true]);
    }

    #[test]
    fn test_xor() {
        assert_eq!(two_bit_gates::xor([true, true], [true, true]), [false, false]);
        assert_eq!(two_bit_gates::xor([true, true], [false, false]), [true, true]);
        assert_eq!(two_bit_gates::xor([true, false], [true, false]), [false, false]);
        assert_eq!(two_bit_gates::xor([false, false], [false, false]), [false, false]);
    }

    #[test]
    fn test_nand() {
        assert_eq!(two_bit_gates::nand([true, true], [true, true]), [false, false]);
        assert_eq!(two_bit_gates::nand([true, true], [false, false]), [true, true]);
        assert_eq!(two_bit_gates::nand([true, false], [true, false]), [false, true]);
        assert_eq!(two_bit_gates::nand([false, false], [false, false]), [true, true]);
    }

    #[test]
    fn test_nor() {
        assert_eq!(two_bit_gates::nor([true, true], [true, true]), [false, false]);
        assert_eq!(two_bit_gates::nor([true, true], [false, false]), [false, false]);
        assert_eq!(two_bit_gates::nor([true, false], [true, false]), [false, true]);
        assert_eq!(two_bit_gates::nor([false, false], [false, false]), [true, true]);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(two_bit_gates::xnor([true, true], [true, true]), [true, true]);
        assert_eq!(two_bit_gates::xnor([true, true], [false, false]), [false, false]);
        assert_eq!(two_bit_gates::xnor([true, false], [true, false]), [true, true]);
        assert_eq!(two_bit_gates::xnor([false, false], [false, false]), [true, true]);
    }
}
