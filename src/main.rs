fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("test", "test");
    }
}
