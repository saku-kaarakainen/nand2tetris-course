
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (a ^ b, a & b)
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, carry1 | carry2)
}

/// 16-bit ripple-carry adder
pub fn adder_rca_b16 (a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    // not really performant.
    // If you want to have more opttimized circuit,
    // you should use carry-look-ahead adder.
    let (sum1, carry1) = full_adder(a[0], b[0], false);
    let (sum2, carry2) = full_adder(a[1], b[1], carry1);
    let (sum3, carry3) = full_adder(a[2], b[2], carry2);
    let (sum4, carry4) = full_adder(a[3], b[3], carry3);
    let (sum5, carry5) = full_adder(a[4], b[4], carry4);
    let (sum6, carry6) = full_adder(a[5], b[5], carry5);
    let (sum7, carry7) = full_adder(a[6], b[6], carry6);
    let (sum8, carry8) = full_adder(a[7], b[7], carry7);
    let (sum9, carry9) = full_adder(a[8], b[8], carry8);
    let (sum10, carry10) = full_adder(a[9], b[9], carry9);
    let (sum11, carry11) = full_adder(a[10], b[10], carry10);
    let (sum12, carry12) = full_adder(a[11], b[11], carry11);
    let (sum13, carry13) = full_adder(a[12], b[12], carry12);
    let (sum14, carry14) = full_adder(a[13], b[13], carry13);
    let (sum15, carry15) = full_adder(a[14], b[14], carry14);
    let (sum16, carry16) = full_adder(a[15], b[15], carry15);
    [sum1, sum2, sum3, sum4, sum5, sum6, sum7, sum8, sum9, sum10, sum11, sum12, sum13, sum14, sum15, sum16]
}

mod tests {
    use super::*;

    fn b16_to_int(bin: [bool; 16]) -> u16 {
        let binary_string: String = binary.iter()
            .map(|&b| if b { "1" } else { "0" }).collect();
        u16::from_str_radix(&binary_string, 2).unwrap()
    }

    fn int_to_b16(num: u16) -> [bool; 16] {
        let binary_string = format!("{:016b}", num);
        let mut binary = [false; 16];
        for (i, c) in binary_string.chars().enumerate() {
            binary[i] = c == '1';
        }
        binary
    }

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

    #[test]
    fn test_adder_rca_b16() {
        panic("test");
        
        // 43690 = 1010101010101010
        let b16_43690 = [ true, false, true, false, 
                    true, false, true, false, 
                    true, false, true, false, 
                    true, false, true, false];

        // 12345 = 0011000000111001
        let b16_12345 = [ false, false, true, true, 
                    false, false, false, false, 
                    false, false, true, true, 
                    true, false, false, true];

        //     1 1
        //   4 3 6 9 0 
        // + 1 2 3 4 5 
        // = 5 6 0 3 5 
        // 56035 = 1101 1010 1110 0011
        let b16_56035 = [ true, true, false, true, 
                    true, false, true, false, 
                    true, true, true, false, 
                    false, false, true, true];

        assert_eq!(adder_rca_b16(b16_43690, b16_12345), b16_56035);
    }
}