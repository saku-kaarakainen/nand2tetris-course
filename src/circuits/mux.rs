use crate::gates::basic_gates;

// create a mux circuit with 2 inputs and 1 output
pub fn mux2x1(a: bool, b: bool, sel: bool) -> bool {
    let not_sel = basic_gates::not(sel);
    let a_and_not_sel = basic_gates::and(a, not_sel);
    let b_and_sel = basic_gates::and(b, sel);
    basic_gates::or(a_and_not_sel, b_and_sel)
}

// create a n-bit mux circuit with 2 inputs and 1 output
pub fn muxnx1(a: Vec<bool>, b: Vec<bool>, sel: Vec<bool>) -> Vec<bool> {
    let mut result = Vec::new();
    for i in 0..sel.len() {
        result.push(mux2x1(a[i], b[i], sel[i]))
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mux2x1() {
        /*
            sel out
            0   a
            1   b
         */
        //                a,     b,     sel,    output  
        assert_eq!(mux2x1(false, false, false), false); // 0   0   0   0
        assert_eq!(mux2x1(false, true,  false), false); // 0   1   0   0
        assert_eq!(mux2x1(true,  false, false), true);  // 1   0   0   1
        assert_eq!(mux2x1(true,  true,  false), true);  // 1   1   0   1
        assert_eq!(mux2x1(false, false, true),  false); // 0   0   1   0
        assert_eq!(mux2x1(false, true,  true),  true);  // 0   1   1   1
        assert_eq!(mux2x1(true,  false, true),  false); // 1   0   1   0
        assert_eq!(mux2x1(true,  true,  true),  true);  // 1   1   1   1
    }

    #[test]
    fn test_muxnx1() {
        /*
            sel out
            0   a
            1   b
         */
        //                a,     b,     sel,    output  
        assert_eq!(muxnx1(vec![false, false], vec![false, false], vec![false, false]), vec![false, false]); // 0   0   0   0
        assert_eq!(muxnx1(vec![false, true],  vec![false, true],  vec![false, false]), vec![false, true]);  // 0   1   0   0
        assert_eq!(muxnx1(vec![true,  false], vec![true,  false], vec![false, false]), vec![true,  false]); // 1   0   0   1
        assert_eq!(muxnx1(vec![true,  true],  vec![true,  true],  vec![false, false]), vec![true,  true]);  // 1   1   0   1
        assert_eq!(muxnx1(vec![false, false], vec![false, false], vec![false, true]),  vec![false, false]); // 0   0   1   0
        assert_eq!(muxnx1(vec![false, true],  vec![false, true],  vec![false, true]),  vec![false, true]);  // 0   1   1   1
        assert_eq!(muxnx1(vec![true,  false], vec![true,  false], vec![false, true]),  vec![true,  false]); // 1   0   1   0
        assert_eq!(muxnx1(vec![true,  true],  vec![true,  true],  vec![false, true]),  vec![true,  true]);  // 1   1   1   1
    }
}