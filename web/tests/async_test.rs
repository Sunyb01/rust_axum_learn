use std::future::Future;

// 异步函数
async fn foo() -> u8 {
    1
}
// 返回一个异步函数
fn bar() -> impl Future<Output = u8> {
    async { 2 }
}

// 返回一个异步函数
fn bar_move() -> impl Future<Output = u8> {
    // 使用 move 关键字, 转移所有权
    async move { 2 }
}

// async
async fn blocks() {
    let my_string = "foo".to_string();
    let feture_one = async {
        println!("{}", my_string);
    };

    let future_two = async {
        println!("{}", my_string);
    };
    // 如果需要使用到锁, 请使用futures::lock, 而不是使用std::sync中的锁; 可能会出现死锁;
    // 因为futures::lock中的锁是异步的, 而std::sync中的锁是同步的;
    // 异步的锁会导致线程阻塞, 而同步的锁不会;
    let ((), ()) = futures::join!(feture_one, future_two);
}

#[cfg(test)]
mod test {

    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_async() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(foo());
        println!("结果是: {}", result);
    }

    #[test]
    fn test_return_async() {
        let rt = Runtime::new().unwrap();
        let f = bar();
        let result = rt.block_on(f);
        println!("结果是: {}", result);
    }

    #[test]
    fn test_return_nove_async() {
        let rt = Runtime::new().unwrap();
        let f = bar_move();
        let result = rt.block_on(f);
        println!("结果是: {}", result);
    }

    #[test]
    fn test_blocks() {
        let rt = Runtime::new().unwrap();
        rt.block_on(blocks());
    }
}
