#[cfg(test)]
mod test {
    use std::io::Error;

    #[test]
    fn test_iter() {
        let array = vec![1, 2, 3, 4, 5];
        match find_element_index(&array, 3) {
            Some(index) => println!("found index = {index}"),
            None => println!("Not found"),
        }
        match find_element_index(&array, 7) {
            Some(index) => println!("found index = {index}"),
            None => println!("Not found"),
        }
    }

    fn find_element_index(array: &[i32], target: i32) -> Option<usize> {
        for (index, value) in array.iter().enumerate() {
            if *value == target {
                return Some(index);
            }
        }

        None
    }

    #[test]
    fn test_result_unwarp1() {
        let ok: Result<i32, &str> = Ok(32);
        let value = ok.unwrap();
        println!("ok = {value}");
        // let err: Result<i32, &str> = Err("ff");
        // let value = err.unwrap();
        // println!("err = {value}");
    }

    #[test]
    fn test_result_unwarp2() {
        let ok: Result<i32, Error> = use_mark_question_4_result();
        let value = ok.unwrap();
        println!("ok = {value}");
    }

    fn use_mark_question_4_result() -> Result<i32, Error> {
        let ok: Result<i32, Error> = Ok(32);
        let value = ok?;
        Ok(value)
    }

    #[test]
    fn test_option_unwarp() {
        let ok: Option<i32> = use_mark_question_4_option();
        let value = ok.unwrap();
        println!("ok = {value}");
    }

    fn use_mark_question_4_option() -> Option<i32> {
        let ok: Option<i32> = Some(32);
        let value = ok?;
        Some(value)
    }
}
