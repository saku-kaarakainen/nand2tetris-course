use crate::gates::basic_gates;


pub fn demux2x1(a: bool, sel: bool) -> (bool, bool) {
    let not_sel = basic_gates::not(sel);
    let a_and_not_sel = basic_gates::and(a, not_sel);
    let a_and_sel = basic_gates::and(a, sel);
    (a_and_not_sel, a_and_sel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demux2x1() {
        /*
            input-a sel output-a output-b
            a       0   0        a
            a       1   a        0  
         */
        //                a,     sel,    output  
        assert_eq!(demux2x1(false, false), (false, false)); // 0   0   0   0
        assert_eq!(demux2x1(false, true),  (false, false)); // 0   1   0   0
        assert_eq!(demux2x1(true,  false), (true,  false)); // 1   0   0   1
        assert_eq!(demux2x1(true,  true),  (false, true));  // 1   1   1   0
    }
}