// 导入 routers_hello 函数
use mactix::router::routers_hello;

#[cfg(test)]
mod tests {
    // 使用父模块, import_mod_test模块
    use super::*;

    #[test]
    fn test_import1() {
        routers_hello();
    }
}
