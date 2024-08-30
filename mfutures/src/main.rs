///
/// futures lib test
///
use futures::{
    executor::{block_on, ThreadPool, ThreadPoolBuilder},
    future::Future,
    task::{Context, Poll},
};

use std::pin::Pin;
use std::thread;

struct Yield {
    rem: usize,
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.rem == 0 {
            println!("{}: done", thread::current().name().unwrap());
            Poll::Ready(())
        } else {
            println!("self.rem={}", self.rem);
            self.rem -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

const NUM: usize = 10;

fn main() {
    // 使用 block_on 函数
    for _ in 0..NUM {
        let y = Yield { rem: NUM };
        block_on(y);
    }
    // 使用 ThreadPool
    for _ in 0..NUM {
        let y = Yield { rem: NUM };
        let pool: ThreadPool = ThreadPoolBuilder::new()
            .name_prefix("pool-")
            .create()
            .unwrap();
        pool.spawn_ok(y);
    }
}
