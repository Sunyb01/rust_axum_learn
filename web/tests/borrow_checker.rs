#[cfg(test)]
mod test {

    #[test]
    fn test_ref() {
        // 可变变量
        let mut s = String::from("hello");
        // 不可变引用
        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
        // 可变引用
        let r3 = &mut s;
        println!("{}", r3);
        // cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here (12行)
        // println!("{}, {}", r1, r2);
        // 不可变变量
        let y = String::from("xxx");
        // y = String::from("yyy"); // cannot mutate immutable variable `y`
        println!("{}", y);
    }
}
