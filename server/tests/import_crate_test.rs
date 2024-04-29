#[cfg(test)]
mod tests {
    // 以下两种都是适用于库crate导入的
    // 扩展使用当前的crate server
    extern crate server;
    // 与上面一致, 其中的"::"" 告诉rust从根目录开始寻找, 即从axum_learn开始
    // use ::server;

    #[test]
    fn test_hello() {
        server::hello();
    }
}
