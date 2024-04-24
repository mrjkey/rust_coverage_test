#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 3), -1);
        assert_eq!(sub(3, 2), 1);
    }
}
