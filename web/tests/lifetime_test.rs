#[cfg(test)]
mod tests {

    // 'a: 'c 表示'a的生命周期大于'c的生命周期
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
}
