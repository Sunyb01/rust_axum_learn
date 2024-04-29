#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }

        fn iter(&self) -> std::slice::Iter<T> {
            self.items.iter()
        }

        fn iter_mut(&mut self) -> std::slice::IterMut<T> {
            self.items.iter_mut()
        }

        fn into_iter(self) -> std::vec::IntoIter<T> {
            self.items.into_iter()
        }
    }

    #[test]
    fn test_custom_iter_push() {
        let mut s1 = Stack::<i32>::new();
        s1.push(1);
        s1.push(2);
        s1.push(3);
        println!("{:?}", s1);
    }

    #[test]
    fn test_custom_iter_pop() {
        let mut s1 = Stack::<i32>::new();
        s1.push(1);
        s1.push(2);
        s1.push(3);
        println!("{:?}", s1);
        s1.pop();
        println!("{:?}", s1);
    }

    #[test]
    fn test_custom_iter() {
        let mut s1 = Stack::<i32>::new();
        s1.push(1);
        s1.push(2);
        s1.push(3);
        let s2 = s1.iter().map(|&i| i * 2).collect::<Vec<i32>>();
        println!("{:?}", s2);
    }

    #[test]
    fn test_custom_iter_mut() {
        let mut s1 = Stack::<i32>::new();
        s1.push(1);
        s1.push(2);
        s1.push(3);
        let s2 = s1.iter_mut().map(|&mut i| i * 2).collect::<Vec<i32>>();
        println!("{:?}", s2);
    }

    #[test]
    fn test_custom_into_iter() {
        let mut s1 = Stack::<i32>::new();
        s1.push(1);
        s1.push(2);
        s1.push(3);
        let s2 = s1.into_iter().map(|i| i * 2).collect::<Vec<i32>>();
        println!("{:?}", s2);
    }
}
