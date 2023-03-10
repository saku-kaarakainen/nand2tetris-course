use crate::gates::gates_b1::{and, or, xor};

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    (xor(a, b), and(a, b))
}

pub fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, c);
    (sum2, or(carry1, carry2))
}

pub fn adder_rca_lsb_b16(a: [bool; 16], b: [bool; 16], cin: bool) -> ([bool; 16], bool) {
    // bit order here is LSB
    let (sum00, c01) = full_adder(a[0], b[0], cin);
    let (sum01, c02) = full_adder(a[1], b[1], c01);
    let (sum02, c03) = full_adder(a[2], b[2], c02);
    let (sum03, c04) = full_adder(a[3], b[3], c03);
    let (sum04, c05) = full_adder(a[4], b[4], c04);
    let (sum05, c06) = full_adder(a[5], b[5], c05);
    let (sum06, c07) = full_adder(a[6], b[6], c06);
    let (sum07, c08) = full_adder(a[7], b[7], c07);
    let (sum08, c09) = full_adder(a[8], b[8], c08);
    let (sum09, c10) = full_adder(a[9], b[9], c09);
    let (sum10, c11) = full_adder(a[10], b[10], c10);
    let (sum11, c12) = full_adder(a[11], b[11], c11);
    let (sum12, c13) = full_adder(a[12], b[12], c12);
    let (sum13, c14) = full_adder(a[13], b[13], c13);
    let (sum14, c15) = full_adder(a[14], b[14], c14);
    let (sum15, cout) = full_adder(a[15], b[15], c15);

    (
        [
            sum00, sum01, sum02, sum03, sum04, sum05, sum06, sum07, sum08, sum09, sum10, sum11,
            sum12, sum13, sum14, sum15,
        ],
        cout,
    )
}

pub fn inc16(input: [bool; 16]) -> [bool; 16] {
    let (sum, _) = adder_rca_lsb_b16(
        input,
        [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ],
        false,
    );
    sum
}

mod tests {
    use super::*;

    const B16_LSB_0: [bool; 16] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    const B16_LSB_1: [bool; 16] = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    const B16_LSB_2: [bool; 16] = [
        false, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    const B16_LSB_3: [bool; 16] = [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];

    const B16_LSB_43690: [bool; 16] = [
        false, true, false, true, false, true, false, true, false, true, false, true, false, true,
        false, true,
    ];

    const B16_LSB_65534: [bool; 16] = [
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];

    const B16_LSB_65535: [bool; 16] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ];

    fn b16_to_int_lsb(b16: [bool; 16]) -> i32 {
        let mut res: i32 = 0;
        for (i, &bit) in b16.iter().enumerate() {
            if bit {
                let app = res += 1 << i;
            }
        }
        res
    }

    fn int_to_b16_lsb(n: i32) -> [bool; 16] {
        let mut b16 = [false; 16];
        let mut quotient = n;
        let mut index = 0;
        while quotient > 0 {
            b16[index] = quotient % 2 == 1;
            quotient /= 2;
            index += 1;
        }
        b16
    }

    #[test]
    fn test_internal_int_to_b16() {
        // test the internal method
        // in order to make sure correct values

        assert_eq!(int_to_b16_lsb(0), B16_LSB_0);
        assert_eq!(int_to_b16_lsb(1), B16_LSB_1);
        assert_eq!(int_to_b16_lsb(2), B16_LSB_2);
        assert_eq!(int_to_b16_lsb(3), B16_LSB_3);
        assert_eq!(int_to_b16_lsb(43690), B16_LSB_43690);
        assert_eq!(int_to_b16_lsb(65534), B16_LSB_65534);
        assert_eq!(int_to_b16_lsb(65535), B16_LSB_65535);
    }

    #[test]
    fn test_internal_b16_to_int() {
        // test the internal method
        // in order to make sure correct values
        assert_eq!(b16_to_int_lsb(B16_LSB_0), 0);
        assert_eq!(b16_to_int_lsb(B16_LSB_1), 1);
        assert_eq!(b16_to_int_lsb(B16_LSB_2), 2);
        assert_eq!(b16_to_int_lsb(B16_LSB_3), 3);
        assert_eq!(b16_to_int_lsb(B16_LSB_43690), 43690);
        assert_eq!(b16_to_int_lsb(B16_LSB_65534), 65534);
        assert_eq!(b16_to_int_lsb(B16_LSB_65535), 65535);
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }

    // adder test case
    struct atc {
        a: [bool; 16],
        b: [bool; 16],
        o: [bool; 16],
        n: String,
    }

    #[test]
    fn test_adder_rca_b16() {
        // arrange
        let test_cases = vec![
            atc {
                a: int_to_b16_lsb(0),
                b: int_to_b16_lsb(0),
                o: int_to_b16_lsb(0),
                n: String::from("test 1: 0 + 0 = 0"),
            },
            atc {
                a: int_to_b16_lsb(0),
                b: int_to_b16_lsb(1),
                o: int_to_b16_lsb(1),
                n: String::from("test 2: 0 + 1 = 1"),
            },
            atc {
                a: int_to_b16_lsb(1),
                b: int_to_b16_lsb(1),
                o: int_to_b16_lsb(2),
                n: String::from("test 3: 1 + 1 = 2"),
            },
            atc {
                a: int_to_b16_lsb(1),
                b: int_to_b16_lsb(2),
                o: int_to_b16_lsb(3),
                n: String::from("test 4: 1 + 2 = 3"),
            },
            atc {
                a: int_to_b16_lsb(8),
                b: int_to_b16_lsb(8),
                o: int_to_b16_lsb(16),
                n: String::from("test 5: 8 + 8 = 16"),
            },
            atc {
                a: int_to_b16_lsb(999),
                b: int_to_b16_lsb(7777),
                o: int_to_b16_lsb(8776),
                n: String::from("test 6: 999 + 7777 = 8776"),
            },
            atc {
                a: int_to_b16_lsb(16384),
                b: int_to_b16_lsb(49151),
                o: int_to_b16_lsb(65535),
                n: String::from("test 7: 16384 + 49151 = 65535"),
            },
        ];

        for case in test_cases {
            print!("\ntesting {n}...\n", n = case.n);

            // act
            let (res, overflow) = adder_rca_lsb_b16(case.a, case.b, false);

            // debug
            print!(
                "expected: {o}. actual: {res}.\n",
                o = b16_to_int_lsb(case.o),
                res = b16_to_int_lsb(res)
            );

            // assert
            assert_eq!(case.o, res);
            assert_eq!(overflow, false);

            // debug
            print!("Test passed.\n");
        }
    }

    // TODO: test adder_rca_b16 with overflow
    // TODO: test inc16

    #[test]
    fn test_inc() {}
}
