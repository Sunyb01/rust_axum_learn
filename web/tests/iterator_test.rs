#[cfg(test)]
mod tests {

    #[test]
    fn test_iter_4_vec() {
        let v = vec![1, 2, 3, 4, 5];
        // 实现了move的语义, 所有权转移了
        let iter = v.into_iter();
        let sum: i32 = iter.sum();
        println!("sum = {}", sum)
    }

    #[test]
    fn test_iter_4_array() {
        let array = [1, 2, 3, 4, 5];
        let iter = array.iter();
        let sum: i32 = iter.sum();
        println!("sum = {}", sum)
    }

    #[test]
    fn test_iter_4_char() {
        let text = "hello world";
        let iter = text.chars();
        // collect::<String>() 是比目鱼操作符确定结果泛型
        let uppercse = iter.map(|c| c.to_ascii_lowercase()).collect::<String>();
        println!("uppercse = {:?}", uppercse)
    }

    #[test]
    fn test_iter_4_immutable() {
        let array = [1, 2, 3, 4, 5];
        let iter = array.iter();
        let sum: i32 = iter.sum();
        println!("sum = {}", sum)
    }

    #[test]
    fn test_iter_4_mutable() {
        let mut array = [1, 2, 3, 4, 5];
        let iter = array.iter_mut();
        let sum = iter.map(|&mut i| i * 2).collect::<Vec<i32>>();
        println!("sum = {:?}", sum)
    }
}
