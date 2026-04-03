pub fn add_binary(a: String, b: String) -> String {
    let mut res = String::new();

    let mut carry = 0;

    for i in (0..a.len().max(b.len())) {
        let a_c = a.as_bytes().iter().rev().nth(i).copied().unwrap_or(b'0');
        let b_c = b.as_bytes().iter().rev().nth(i).copied().unwrap_or(b'0');

        match (a_c, b_c, carry) {
            (b'0', b'0', 0) => res.push('0'),
            (b'0', b'1', 0) | (b'1', b'0', 0) => res.push('1'),
            (b'1', b'1', 0) => {
                res.push('0');
                carry = 1;
            }
            (b'0', b'0', 1) => {
                res.push('1');
                carry = 0;
            }
            (b'0', b'1', 1) | (b'1', b'0', 1) => res.push('0'),
            (b'1', b'1', 1) => {
                res.push('1');
            }
            _ => unreachable!(),
        }
    }

    if carry == 1 {
        res.push('1');
    }

    res.chars().rev().collect()
}
