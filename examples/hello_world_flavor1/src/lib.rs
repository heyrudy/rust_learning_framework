/// This helper module provide `greeting` function
pub fn greeting() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn test_hello() {
        assert_eq!("Hello, world!", greeting())
    }
}
