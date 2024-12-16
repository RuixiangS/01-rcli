fn main() {
    println!("Hello, rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_test() {
        // 这是一个空的测试，确保 cargo test 不会失败
    }
}
