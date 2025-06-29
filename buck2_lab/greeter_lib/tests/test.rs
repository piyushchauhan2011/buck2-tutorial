#[cfg(test)]
mod tests {
    use greeter_lib;

    #[test]
    fn test_greet() {
        assert_eq!(greeter_lib::greet("World"), "Hello, World!");
        assert_eq!(greeter_lib::greet("Buck2"), "Hello, Buck2!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greeter_lib::greet(""), "Hello, !");
    }
}
