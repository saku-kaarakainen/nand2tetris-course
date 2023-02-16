
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
    let (sum00, carry00) = full_adder(a[0], b[0], false);
    let (sum01, carry01) = full_adder(a[1], b[1], carry00);
    let (sum02, carry02) = full_adder(a[2], b[2], carry01);
    let (sum03, carry03) = full_adder(a[3], b[3], carry02);
    let (sum04, carry04) = full_adder(a[4], b[4], carry03);
    let (sum05, carry05) = full_adder(a[5], b[5], carry04);
    let (sum06, carry06) = full_adder(a[6], b[6], carry05);
    let (sum07, carry07) = full_adder(a[7], b[7], carry06);
    let (sum08, carry08) = full_adder(a[8], b[8], carry07);
    let (sum09, carry09) = full_adder(a[9], b[9], carry08);
    let (sum10, carry10) = full_adder(a[10], b[10], carry09);
    let (sum11, carry11) = full_adder(a[11], b[11], carry10);
    let (sum12, carry12) = full_adder(a[12], b[12], carry11);
    let (sum13, carry13) = full_adder(a[13], b[13], carry12);
    let (sum14, carry14) = full_adder(a[14], b[14], carry13);
    let (sum15, _) = full_adder(a[15], b[15], carry14);
    
    [   sum00, sum01, sum02, sum03, 
        sum04, sum05, sum06, sum07, 
        sum08, sum09, sum10, sum11, 
        sum12, sum13, sum14, sum15]
}

mod tests {
    use super::*;

    fn b16_to_int(bin: [bool; 16]) -> u16 {
        let binary_string: String = bin.iter()
            .map(|&b| if b { "1" } else { "0" }).collect();
        u16::from_str_radix(&binary_string, 2).unwrap()
    }

    // Util method for testing
    fn int_to_b16(num: u16) -> [bool; 16] {
        let binary_string = format!("{:016b}", num);
        let mut binary = [false; 16];
        for (i, c) in binary_string.chars().enumerate() {
            binary[i] = c == '1';
        }
        binary
    }

    #[test]
    fn test_internal_int_to_b16() {
        // test the internal method
        // in order to make sure correct values
        let b16_0 = [ false, false, false, false, 
                                false, false, false, false,
                                false, false, false, false, 
                                false, false, false, false];
        
        let b16_43690 = [ true, false, true, false, 
                                true, false, true, false,
                                true, false, true, false, 
                                true, false, true, false];

        let b16_65535 = [ true, true, true, true, 
                                true, true, true, true,
                                true, true, true, true, 
                                true, true, true, true];
       
        assert_eq!(int_to_b16(0),     b16_0);
        assert_eq!(int_to_b16(43690), b16_43690);
        assert_eq!(int_to_b16(65535), b16_65535);   
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
        let ex_int = 55555;
        let ex = int_to_b16(ex_int);
        let res = adder_rca_b16(
            int_to_b16(22222), 
            int_to_b16(33333));

        let res_as_int = b16_to_int(res);
        print!("\n\nex : {:04x}\n", ex_int);
        print!("res: {:04x}\n\n", res_as_int);

        assert_eq!(ex, res);
    }
}