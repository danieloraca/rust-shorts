use exercises::{check, checker};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_with_positive_number() {
        assert_eq!(check(5), false);
    }

    #[test]
    fn test_check_with_negative_number() {
        assert_eq!(check(-5), false);
    }

    #[test]
    fn test_checker() {
        assert_eq!(checker(), ());
    }
}
