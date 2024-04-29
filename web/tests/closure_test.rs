#[cfg(test)]
mod tests {

    #[test]
    fn test_closure_fn() {
        let fn_func = |s: String| {
            println!("--fn--");
            println!("{s}");
            println!("--fn--");
        };

        fn_func("hello".to_string());
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

    #[test]
    fn test_closure_fn_once() {
        let s1 = String::from("hello");
        let fn_once_func = || {
            println!("--{s1}--");
        };

        fn_once_func();
    }
}
