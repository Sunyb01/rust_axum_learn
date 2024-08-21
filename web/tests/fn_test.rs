/// fn fnMut fnOnce

#[cfg(test)]
mod tests {

    fn return_fn() -> fn() -> () {
        || {
            print!("this is return_fn");
            ()
        }
    }

    #[test]
    fn test_return_fn() {
        let f = return_fn();
        f();
        let f2 = return_fn;
        let f3 = f2();
        f3();
    }

    fn fn_param(f: fn(String) -> ()) {
        f("fn_param".to_string());
    }

    #[test]
    fn test_fn_param() {
        fn_param(|s| print!("hello this is {s}"));
    }

    fn fn_param2<F>(f: F)
    where
        F: Fn(String) -> (),
    {
        f("fn_param2".to_string());
    }

    #[test]
    fn test_fn_param_with_generic() {
        fn_param2(|s| print!("hello this is {s}"));
    }
}
