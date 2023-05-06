#[cfg(test)]
mod tests {
    use crate::process_string;

    #[test]
    fn expression_evaluation() {
        [
            ("1", 1),
            ("-1", -1),
            ("1 + 2", 3),
            ("1 + 2 + 3", 6),
            ("1 * 2", 2),
            ("1 * 2 * 3 * 4", 24),
            ("-5 - 2", -7),
            ("10 / 2", 5),
            ("10 - 2 + 3", 11),
            ("2 + 3 * 4", 14),
            ("2 * 3 + 4", 10),
            ("(2 + 3) * 4", 20),
            ("(2 + 3) * (4 - 1)", 15),
            ("2 ^ 0", 1),
            ("2 ^ 1", 2),
            ("2 ^ 2", 4),
            ("2 ^ 3", 8),
            ("2 ^ 3 + 4", 12),
        ]
        .into_iter()
        .for_each(|(string, val)| {
            assert_eq!(process_string(string.to_string(), false).unwrap(), val);
        })
    }
}
