use rx_state::CONFIG;

fn main() {
    println!("Example bool: {}", CONFIG.example_bool);
    println!("Example list: {:?}", CONFIG.example_list);
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::main;

    /// Run main, it might makes sense to test it sometimes. Here it's mostly to improve coverage.
    #[test]
    fn test_main() {
    }
}
