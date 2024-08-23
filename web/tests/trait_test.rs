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
    // dyn
    fn call_obj4(item: &dyn Overview) {
        let value = item.overview();
        println!("value = {}", value);
    }

    #[test]
    fn test_generics() {
        call_obj(&Obj {});
        call_obj1(&Obj {});
        call_obj2(&Obj {});
        call_obj3(Box::new(&Obj {}));
        call_obj4(&Obj {});
    }

    trait Animal {
        fn make_sound(&self);
    }

    fn get_animal() -> impl Animal {
        struct Dog;
        impl Animal for Dog {
            fn make_sound(&self) {
                println!("Woof!");
            }
        }
        Dog
    }

    fn get_animal2() -> Box<dyn Animal> {
        struct Dog;
        impl Animal for Dog {
            fn make_sound(&self) {
                println!("Woof2!");
            }
        }
        Box::new(Dog)
    }

    #[test]
    fn test_impl() {
        let animal = get_animal();
        animal.make_sound(); // 输出 "Woof!"
        let animal = get_animal2();
        animal.make_sound(); // 输出 "Woof2!"
    }

    trait Animal2 {
        fn make_sound(&self);
    }

    struct Dog2;
    impl Animal2 for Dog2 {
        fn make_sound(&self) {
            println!("Woof!");
        }
    }

    struct Cat2;
    impl Animal2 for Cat2 {
        fn make_sound(&self) {
            println!("Meow!");
        }
    }

    #[test]
    fn test_dyn() {
        let dog: Box<dyn Animal2> = Box::new(Dog2);
        let cat: Box<dyn Animal2> = Box::new(Cat2);

        dog.make_sound(); // 输出 "Woof!"
        cat.make_sound(); // 输出 "Meow!"
    }
}
