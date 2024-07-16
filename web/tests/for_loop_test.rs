#[cfg(test)]
mod test {

    #[test]
    fn not_contains_max() {
        for i in 1..10 {
            println!("i = {}", i)
        }
    }

    #[test]
    fn contains_max() {
        for i in 1..=10 {
            println!("i = {}", i)
        }
    }

    #[test]
    fn loop_vec() {
        let numbers = vec![1, 2, 3];
        for i in numbers {
            println!("i = {}", i)
        }
    }
}
