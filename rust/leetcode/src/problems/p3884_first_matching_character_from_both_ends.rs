pub fn first_matching_index(s: String) -> i32 {
    for i in 0..s.len() {
        if s.as_bytes()[i] == s.as_bytes()[s.len() - i - 1] {
            return i as i32;
        }
    }

    -1
}
