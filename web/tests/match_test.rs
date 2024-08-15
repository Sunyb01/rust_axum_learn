/// 模式匹配
///

fn match1() -> (i32, String) {
    (1, "hello".to_string())
}

fn match2() -> Result<String, std::io::Error> {
    Ok("hello".to_string())
}

pub fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 10u32.pow(3);
    const M: u32 = 10u32.pow(6);
    const G: u32 = 10u32.pow(9);
    match n {
        ..K => "",
        K..M => "k",
        M..G => "M",
        G.. => "G",
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_match_model() {
        // 与Python一样支持模式匹配
        let (age, name) = super::match1();
        println!("age is: {age}, name is: {name}")
    }

    #[test]
    fn test_if_let() {
        let x = super::match2();
        if let Ok(name) = x {
            println!("name is: {name}")
        } else {
            println!("No match")
        }
    }

    #[test]
    fn test_match() {
        let x = super::match2();
        match x {
            Ok(name) => println!("name is: {name}"),
            Err(e) => println!("error is: {e}"),
        }
    }

    #[test]
    fn test_match_range1() {
        let x = 10;
        match x {
            1..=5 => println!("1..=5"),
            6..=10 => println!("6..=10"),
            _ => println!("other"),
        }
    }

    #[test]
    fn test_match_range2() {
        let x = 10;
        match x {
            ..=5 => println!("1..=5"),
            6..=10 => println!("6..=10"),
            _ => println!("other"),
        }
    }

    #[test]
    fn test_match_range3() {
        // Rust现在支持模式中使用排他区间（a..b或..b），
        let r = super::size_prefix(10);
        println!("r is: {r}")
    }
}
