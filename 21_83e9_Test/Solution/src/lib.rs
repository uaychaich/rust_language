mod math;

/// ```
/// let result = testlib::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn addcase1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn addcase2() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn addcase3() {
        let result = add(3, 3);
        assert_eq!(result, 6);
    }
}