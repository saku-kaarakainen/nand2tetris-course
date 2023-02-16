
/// Half Adder circuit.
/// 
/// ## Arguments
/// 
///     * `a` - First input bit
///     * `b` - Second input bit
/// 
/// ## Returns
///     * `sum` - Sum of the two input bits
///     * `carry` - Carry bit
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (a ^ b, a & b)
}

/// Full Adder circuit.
/// 
/// ## Arguments
/// 
///     * `a` - First input bit
///     * `b` - Second input bit
///     * `c` - Carry bit
/// 
/// ## Returns
///    * `sum` - Sum of the two input bits
///    * `carry` - Carry bit
pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, carry1 | carry2)
}

// Util method for testing
fn b16_to_u16_lsb(b16: [bool; 16]) -> u16 {
   let b_str: String = b16.iter().rev()
        .map(|&b| if b { '1' } else { '0' })
        .collect();
    u16::from_str_radix(&b_str, 2).unwrap()
}

// Util method for testing
fn u16_to_b16_lsb(val: u16) -> [bool; 16] {
    let b_str = format!("{:016b}", val);
    let mut b16 = [false; 16];
    for (i, c) in b_str.chars().rev().enumerate() {
        b16[i] = c == '1';
    }
    b16
}

/// 16-bit ripple-carry adder. The bit order is MSB first.
/// 
/// ## Arguments
/// 
///     * `a` - First input bit
///     * `b` - Second input bit
///     * `cin` - Carry bit
/// 
/// ## Returns
///   * `sum` - Sum of the two input bits
///   * `carry_out` - Carry bit
pub fn adder_rca_lsb_b16 (
    a: [bool; 16], 
    b: [bool; 16],
    cin: bool
) -> ([bool; 16], bool) {
    // bit order here is 
    let (sum00, c01) = full_adder(a[0],  b[0],  cin);
    print!("a[0]:{}, b[0]:{}, cin:{} => o[0]:{} c01:{}\n", a[0], b[0], cin, sum00, c01);

    let (sum01, c02) = full_adder(a[1],  b[1],  c01);
    print!("a[1]:{}, b[1]:{}, c01:{} => o[1]:{} c02:{}\n", a[1], b[1], c01, sum01, c02);

    let (sum02, c03) = full_adder(a[2],  b[2],  c02);
    print!("a[2]:{}, b[2]:{}, c02:{} => o[2]:{} c03:{}\n", a[2], b[2], c02, sum02, c03);

    let (sum03, c04) = full_adder(a[3],  b[3],  c03);
    print!("a[3]:{}, b[3]:{}, c03:{} => o[3]:{} c04:{}\n", a[3], b[3], c03, sum03, c04);

    let (sum04, c05) = full_adder(a[4],  b[4],  c04);
    print!("a[4]:{}, b[4]:{}, c04:{} => o[4]:{} c05:{}\n", a[4], b[4], c04, sum04, c05);

    let (sum05, c06) = full_adder(a[5],  b[5],  c05);
    let (sum06, c07) = full_adder(a[6],  b[6],  c06);
    let (sum07, c08) = full_adder(a[7],  b[7],  c07);
    let (sum08, c09) = full_adder(a[8],  b[8],  c08);
    let (sum09, c10) = full_adder(a[9],  b[9],  c09);
    let (sum10, c11) = full_adder(a[10], b[10], c10);
    let (sum11, c12) = full_adder(a[11], b[11], c11);
    let (sum12, c13) = full_adder(a[12], b[12], c12);
    let (sum13, c14) = full_adder(a[13], b[13], c13);
    let (sum14, c15) = full_adder(a[14], b[14], c14);
    let (sum15, cout) = full_adder(a[15], b[15], c15);

    ([  sum00, sum01, sum02, sum03, 
        sum04, sum05, sum06, sum07, 
        sum08, sum09, sum10, sum11, 
        sum12, sum13, sum14, sum15
    ], cout)
}



mod tests {
    use super::*;

    const B16_LSB_0: [bool; 16] = [ 
        false, false, false, false, 
        false, false, false, false,
        false, false, false, false, 
        false, false, false, false];

    const B16_LSB_1: [bool; 16] = [ 
        true, false, false, false, 
        false, false, false, false,
        false, false, false, false, 
        false, false, false, false];

    const B16_LSB_2: [bool; 16] = [ 
        false, true, false, false, 
        false, false, false, false,
        false, false, false, false, 
        false, false, false, false];

    const B16_LSB_3: [bool; 16] = [
        true, true, false, false, 
        false, false, false, false,
        false, false, false, false, 
        false, false, false, false];

    const B16_LSB_43690: [bool; 16] = [ 
        false, true, false, true,
        false, true, false, true,
        false, true, false, true,
        false, true, false, true];

    const B16_LSB_65534: [bool; 16] = [ 
        true, true, true, true, 
        true, true, true, true,
        true, true, true, true, 
        true, true, true, false];

    const B16_LSB_65535: [bool; 16] = [ 
        true, true, true, true, 
        true, true, true, true,
        true, true, true, true, 
        true, true, true, true];

    #[test]
    fn test_internal_u16_to_b16() {
        // test the internal method
        // in order to make sure correct values
        assert_eq!(u16_to_b16_lsb(0),     B16_LSB_0);
        assert_eq!(u16_to_b16_lsb(1),     B16_LSB_1);
        assert_eq!(u16_to_b16_lsb(2),     B16_LSB_2);
        assert_eq!(u16_to_b16_lsb(3),     B16_LSB_3);
        assert_eq!(u16_to_b16_lsb(43690), B16_LSB_43690);
        assert_eq!(u16_to_b16_lsb(65535), B16_LSB_65534);   
        assert_eq!(u16_to_b16_lsb(65535), B16_LSB_65535);   
    }

    #[test]
    fn test_internal_b16_to_int() {
        // test the internal method
        // in order to make sure correct values       
        assert_eq!(b16_to_u16_lsb(B16_LSB_0),     0);
        print!("ok\n");

        
        assert_eq!(b16_to_u16_lsb(B16_LSB_1),     1);
        print!("1ok\n");
        
        assert_eq!(b16_to_u16_lsb(B16_LSB_2),     2);
        print!("o2k\n");
        
        assert_eq!(b16_to_u16_lsb(B16_LSB_3),     3);
        print!("3ok\n");
        
        assert_eq!(b16_to_u16_lsb(B16_LSB_43690), 43690);
        print!("o4k\n");
        
        assert_eq!(b16_to_u16_lsb(B16_LSB_65535), 65534);   
        print!("o5k\n");
        
        assert_eq!(b16_to_u16_lsb(B16_LSB_65535), 65535);   
        print!("6ok\n");
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

    // adder test case
    struct atc {
        a: [bool; 16],
        b: [bool; 16],
        o: [bool; 16],
        n: String
    }

/*
    #[test]
    fn test_adder_rca_b16_case1() {
        // arrange
        let test_cases = vec![
            atc { a:u16_to_b16_lsb(0),      b:u16_to_b16_lsb(0),     o:u16_to_b16_lsb(0),     n: String::from("test 1: 0 + 0 = 0") }, 
            atc { a:u16_to_b16_lsb(0),      b:u16_to_b16_lsb(1),     o:u16_to_b16_lsb(1),     n: String::from("test 2: 0 + 1 = 1") }, 
            atc { a:u16_to_b16_lsb(1),      b:u16_to_b16_lsb(1),     o:u16_to_b16_lsb(2),     n: String::from("test 3: 1 + 1 = 2") }, 
            atc { a:u16_to_b16_lsb(1),      b:u16_to_b16_lsb(2),     o:u16_to_b16_lsb(3),     n: String::from("test 4: 1 + 2 = 3") }, 
            atc { a:u16_to_b16_lsb(8),      b:u16_to_b16_lsb(8),     o:u16_to_b16_lsb(16),    n: String::from("test 5: 8 + 8 = 16") }, 
            atc { a:u16_to_b16_lsb(999),    b:u16_to_b16_lsb(7777),  o:u16_to_b16_lsb(8776),  n: String::from("test 6: 999 + 7777 = 8776") }, 
            atc { a:u16_to_b16_lsb(16384),  b:u16_to_b16_lsb(49151), o:u16_to_b16_lsb(65535), n: String::from("test 7: 16384 + 49151 = 65535") }, 
        ];

        for case in test_cases{
            print!("\ntesting {n}...\n", n=case.n);

            // act
            let (res, overflow) = adder_rca_lsb_b16(case.a, case.b, false);

            // debug 
            print!("expected: {o}. actual: {res}.\n", o=b16_to_u16_lsb(case.o), res=b16_to_u16_lsb(res));

            // assert
            assert_eq!(case.o, res);
            assert_eq!(overflow, false);

            // debug
            print!("Test passed.\n");
        }
    }*/
}