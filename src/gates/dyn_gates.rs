// Create a library of Logical x-bit Binary Gates, such as AND, OR, NOT, XOR, etc.
// input must be unknown size binary array that has unknown amount of elements.

mod dyn_gates{
    // AND Gate
    pub fn and(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(a[i] && b[i]);
        }
        result
    }

    // OR Gate
    pub fn or(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(a[i] || b[i]);
        }
        result
    }

    // NOT Gate
    pub fn not(a: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(!a[i]);
        }
        result
    }

    // XOR Gate
    pub fn xor(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(a[i] ^ b[i]);
        }
        result
    }

    // NAND Gate
    pub fn nand(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(!(a[i] && b[i]));
        }
        result
    }

    // NOR Gate
    pub fn nor(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(!(a[i] || b[i]));
        }
        result
    }

    // XNOR Gate
    pub fn xnor(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
        let mut result = Vec::new();
        for i in 0..a.len() {
            result.push(!(a[i] ^ b[i]));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(dyn_gates::and(vec![true, true], vec![true, true]), vec![true, true]);
        assert_eq!(dyn_gates::and(vec![true, true], vec![false, false]), vec![false, false]);
        assert_eq!(dyn_gates::and(vec![true, false], vec![true, false]), vec![true, false]);
        assert_eq!(dyn_gates::and(vec![false, false], vec![false, false]), vec![false, false]);
    }

    #[test]
    fn test_or() {
        assert_eq!(dyn_gates::or(vec![true, true], vec![true, true]), vec![true, true]);
        assert_eq!(dyn_gates::or(vec![true, true], vec![false, false]), vec![true, true]);
        assert_eq!(dyn_gates::or(vec![true, false], vec![true, false]), vec![true, false]);
        assert_eq!(dyn_gates::or(vec![false, false], vec![false, false]), vec![false, false]);
    }

    #[test]
    fn test_not() {
        assert_eq!(dyn_gates::not(vec![true, true]), vec![false, false]);
        assert_eq!(dyn_gates::not(vec![true, false]), vec![false, true]);
        assert_eq!(dyn_gates::not(vec![false, false]), vec![true, true]);
        assert_eq!(dyn_gates::not(vec![false, true]), vec![true, false]);
    }

    #[test]
    fn test_xor() {
        assert_eq!(dyn_gates::xor(vec![true, true], vec![true, true]), vec![false, false]);
        assert_eq!(dyn_gates::xor(vec![true, true], vec![false, false]), vec![true, true]);
        assert_eq!(dyn_gates::xor(vec![true, false], vec![true, false]), vec![false, false]);
        assert_eq!(dyn_gates::xor(vec![false, false], vec![false, false]), vec![false, false]);
    }

    #[test]
    fn test_nand() {
        assert_eq!(dyn_gates::nand(vec![true, true], vec![true, true]), vec![false, false]);
        assert_eq!(dyn_gates::nand(vec![true, true], vec![false, false]), vec![true, true]);
        assert_eq!(dyn_gates::nand(vec![true, false], vec![true, false]), vec![false, true]);
        assert_eq!(dyn_gates::nand(vec![false, false], vec![false, false]), vec![true, true]);
    }

    #[test]
    fn test_nor() {
        assert_eq!(dyn_gates::nor(vec![true, true], vec![true, true]), vec![false, false]);
        assert_eq!(dyn_gates::nor(vec![true, true], vec![false, false]), vec![false, false]);
        assert_eq!(dyn_gates::nor(vec![true, false], vec![true, false]), vec![false, true]);
        assert_eq!(dyn_gates::nor(vec![false, false], vec![false, false]), vec![true, true]);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(dyn_gates::xnor(vec![true, true], vec![true, true]), vec![true, true]);
        assert_eq!(dyn_gates::xnor(vec![true, true], vec![false, false]), vec![false, false]);
        assert_eq!(dyn_gates::xnor(vec![true, false], vec![true, false]), vec![true, true]);
        assert_eq!(dyn_gates::xnor(vec![false, false], vec![false, false]), vec![true, true]);
    }

    
}
