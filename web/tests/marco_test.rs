#[cfg(test)]
mod tests {

    #[warn(unused_unsafe)]
    macro_rules! syscall {
        // ident 类型的参数fn 函数名
        // $($arg: expr),* 标识多个参数, 其中逗号进行分割; $(...)表示一个参数整体也就是 ==> $(id: i32), $(name: String); 而其中的 id: i32 ==> 赋值给 arg这个参数, 参数的类型为expr;
        // $(,)* 表示逗号可以出现多次
        ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
            let res = $fn($($arg, )*);
            println!("res: {:?}", res);
        }};
    }

    fn my_fn(id: i32, name: String) -> Result<(), std::io::Error> {
        print!("id = {}, name = {}\n", id, name);
        Ok(())
    }

    #[test]
    fn test_marco() {
        syscall!(my_fn(1, "hello".to_string(),))
    }
}
