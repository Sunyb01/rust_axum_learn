#[cfg(test)]
mod tests {

    // 'a: 'c 表示'a的至少要活的和'c一样长
    fn longer_lifetiem<'a, 'b, 'c>(s1: &'a str, s2: &'b str) -> &'c str
    where
        'a: 'c,
        'b: 'c,
    {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    fn longer_lifetiem2<'a: 'c, 'b: 'c, 'c>(s1: &'a str, s2: &'b str) -> &'c str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    #[test]
    fn test_lifetime() {
        let s1 = String::from("hello");
        let s2 = String::from("hello");
        let s = longer_lifetiem(&s1, &s2);
        println!("{s}");
        let s1 = String::from("hello");
        let s2 = String::from("hello");
        let s = longer_lifetiem2(&s1, &s2);
        println!("{s}");
    }

    #[test]
    fn test_lifetime_mut() {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("word = {}", word); // 解开下面的打印, 同时注释掉这行
                                     // word 也是指向 s对应的字符串引用, 这里就有了2个不可变引用, 分别是s, word;
                                     //  但是由于s.clear()后面没有其他引用, 不会造成数据的不一致, 所以编译通过;
                                     // s.clear() 改写为String::clear(&mut s)就比较容易理解了
        s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
                   //  解开这行就会报错; 原因就是这里就造成了, 同一个作用域内同时拥有了可变借用与不可变借用的问题;
                   // println!("word = {}", word);
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..index];
            }
        }

        &s[..]
    }

    #[derive(Debug)]
    struct M1<'a> {
        // 生命周期 'a 的意义在于确保 M1 中的 s 引用不会超过 'a 生命周期的长度。
        // 在这个例子中，'a 生命周期的长度是由 m 的生命周期决定的，因为 s 引用是在 m 构造时创建的，并且 m 是在 test_struct_lifetime 函数的作用域内创建的。
        s: &'a String,
        n: i32,
    }

    fn x(m: &M1) {
        println!("s={}, n={}", m.s, m.n);
    }

    #[test]
    fn test_struct_lifetime() {
        let hello = String::from("hello");
        let s: &String = &hello;
        let m = M1 { s, n: 1 };
        x(&m);
    }
}
