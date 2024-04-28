#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct MyError {
        detail: String,
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Custom Error: {}", self.detail)
        }
    }

    impl std::error::Error for MyError {
        fn description(&self) -> &str {
            &self.detail
        }
    }

    #[test]
    fn test_cutom_error() {
        let my_err = return_my_error();
        match my_err {
            Ok(_) => println!("ok"),
            Err(err) => println!("{}", err),
        }
    }

    fn return_my_error() -> Result<i32, MyError> {
        Err(MyError {
            detail: String::from("my error"),
        })
    }
}
