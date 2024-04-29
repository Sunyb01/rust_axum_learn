#[cfg(test)]
mod tests {
    /// Fn -> FnMut -> FnOnce
    #[test]
    fn test_closure_fn() {
        let fn_func = |s: String| {
            println!("--fn--");
            println!("{s}");
            println!("--fn--");
        };

        fn_func("hello".to_string());
    }

    fn closure_fn<F>(func: F)
    where
        F: Fn(),
    {
        func();
        func();
    }

    #[test]
    fn test_closure_fn2() {
        let fn_func = || {
            println!("--fn--");
        };

        closure_fn(fn_func);
    }

    #[test]
    fn test_closure_fn_mut() {
        let mut s1 = String::from("hello");
        let mut fn_mut_func = |s: String| {
            s1.push_str("world");
            println!("--fn--");
            println!("{s}");
            println!("--fn--");
            println!("--{s1}--");
        };

        fn_mut_func(String::from("value"));
    }

    fn closure_fn_mut<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
        func();
    }

    #[test]
    fn test_closure_fn_mut2() {
        let mut s1 = String::from("hello");
        let fn_mut_func = || {
            s1.push_str("string");
            println!("{}", s1);
        };
        closure_fn_mut(fn_mut_func);
    }

    #[test]
    fn test_closure_fn_once() {
        let s1 = String::from("hello");
        let fn_once_func = || {
            println!("--{s1}--");
            std::mem::drop(s1);
        };

        fn_once_func();
    }
}
