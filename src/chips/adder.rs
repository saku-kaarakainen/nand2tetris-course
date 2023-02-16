
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (a ^ b, a & b)
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, carry1 | carry2)
}

mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true),  (true, false));
        assert_eq!(half_adder(true,  false), (true, false));
        assert_eq!(half_adder(true,  true),  (false, true));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true),  (true,  false));
        assert_eq!(full_adder(false, true,  false), (true,  false));
        assert_eq!(full_adder(false, true,  true),  (false, true));
        assert_eq!(full_adder(true,  false, false), (true,  false));
        assert_eq!(full_adder(true,  false, true),  (false, true));
        assert_eq!(full_adder(true,  true,  false), (false, true));
        assert_eq!(full_adder(true,  true,  true),  (true,  true)); 
    }
}