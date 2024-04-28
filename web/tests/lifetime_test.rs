#[cfg(test)]
mod test {
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
}
