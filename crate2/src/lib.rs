pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn number() -> i32 {
    crate1::one()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_number() {
        assert_eq!(number(), 1);
    }
}
