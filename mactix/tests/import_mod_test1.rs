#[cfg(test)]
mod tests {

    use mactix::router::routers_hello;

    #[test]
    fn test_dummy() {
        // 可以直接使用库名::mod;
        mactix::router::routers_hello();
        // 这两种是等效的
        routers_hello();
    }

    #[actix_rt::test]
    async fn health_check_test() {
        // let response = mactix::health_check().await;
        // assert!(response.status().is_success())
    }
}
