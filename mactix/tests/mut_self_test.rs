///
/// 测试 mut self
///

struct MyStruct1 {
    age: i32,
}

impl MyStruct1 {
    fn new(age: i32) -> MyStruct1 {
        MyStruct1 { age }
    }
    // 这里的mut self 发生了所有权转移
    fn add_age(mut self) {
        self.age += 1;
    }

    // fn add_age2(&self) {
    //     // cannot assign to `self.age`, which is behind a `&` reference `self` is a `&` reference, so the data it refers to cannot be written
    //     self.age += 1;
    // }

    fn add_age3(&mut self) {
        self.age += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mut_self() {
        let m = MyStruct1::new(10);
        m.add_age();
        // borrow of moved value: `m` value borrowed here after move
        // assert_eq!(m.age, 11);
        let mut m = MyStruct1::new(10);
        m.add_age3();
        assert_eq!(m.age, 11);
    }
}
