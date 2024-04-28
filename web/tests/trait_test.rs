#[cfg(test)]
mod test {
    struct Obj {}

    trait Overview {
        fn overview(&self) -> String {
            String::from("Overview")
        }
    }

    impl Overview for Obj {
        fn overview(&self) -> String {
            String::from("Obj")
        }
    }
    // 规定实现
    fn call_obj(item: &impl Overview) {
        let value = item.overview();
        println!("value = {}", value);
    }
    // 泛型1
    fn call_obj1<T: Overview>(item: &T) {
        let value = item.overview();
        println!("value = {}", value);
    }
    // 泛型2
    fn call_obj2<T>(item: &T)
    where
        T: Overview,
    {
        let value = item.overview();
        println!("value = {}", value);
    }
    // Box
    fn call_obj3(item: Box<&dyn Overview>) {
        let value = item.overview();
        println!("value = {}", value);
    }

    #[test]
    fn test_generics() {
        call_obj(&Obj {});
        call_obj1(&Obj {});
        call_obj2(&Obj {});
        call_obj3(Box::new(&Obj {}));
    }
}
