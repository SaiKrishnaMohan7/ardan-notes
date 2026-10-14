async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_sync_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_double_hard_way() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test]
    async fn test_double() {
        assert_eq!(double(2).await, 4);
    }
}
