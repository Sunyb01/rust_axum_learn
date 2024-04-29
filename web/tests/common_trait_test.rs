#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct MyStruct(i32);

    impl From<i32> for MyStruct {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl Into<i32> for MyStruct {
        fn into(self) -> i32 {
            self.0
        }
    }

    #[test]
    fn test_from_trait() {
        let value = 5;
        let m: MyStruct = MyStruct::from(value);
        println!("m = {:?}", m);
    }
    #[test]
    fn test_into_trait() {
        let m: MyStruct = MyStruct::from(6);
        let value: i32 = m.into();
        println!("m = {:?}", value);
    }
}
