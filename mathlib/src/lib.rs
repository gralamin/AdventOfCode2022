// modulus function.
pub fn modulus(a: i32, b: i32) -> i32 {
    // % is actually the remainder function, not the modulus function
    // This is the workaround way to "fix" this.
    return ((a % b) + b) % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_one_neg() {
        let result = modulus(-2, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn modulus_both_neg() {
        let result = modulus(-2, -3);
        assert_eq!(result, -2);
    }

    #[test]
    fn modulus_both_pos() {
        let result = modulus(120, 3);
        assert_eq!(result, 0);
    }
}
