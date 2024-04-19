#[cfg(test)]
mod test {

    #[test]
    fn test_reference() {
        let c1 = Counter::new(1);
        let c2 = Counter::new(2);
        let c3 = Counter::combine2(&c1, &c2);
        println!("c1 = {:?}", c1);
        println!("c2 = {:?}", c2);
        println!("c3 = {:?}", c3);
    }

    #[test]
    fn test_ownership_changed() {
        let c1 = Counter::new(1);
        let c2 = Counter::new(2);
        let c3 = Counter::combine(c1, c2);
        // println!("c1 = {:?}", c1); // borrow of moved value: `c1`
        // println!("c2 = {:?}", c2); // borrow of moved value: `c2``
        println!("c3 = {:?}", c3);
    }

    #[derive(Debug)]
    struct Counter {
        number: i32,
    }

    impl Counter {
        fn new(number: i32) -> Self {
            Counter { number }
        }

        fn get_number(&self) -> i32 {
            return self.number;
        }

        fn combine(c1: Counter, c2: Counter) -> Self {
            Counter {
                number: c1.get_number() + c2.get_number(),
            }
        }

        fn combine2(c1: &Counter, c2: &Counter) -> Self {
            Counter {
                number: c1.get_number() + c2.get_number(),
            }
        }
    }
}
