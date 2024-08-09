struct MyStruct {
    name: String,
}

fn fn_ref_test(m: &mut MyStruct) {
    // 与Java、Python等语言不同, 这里可以直接更改参数的数据本身
    // m本身代表了一个 指针对象; *m 这个解引用操作, 直接获取到了MyStruct对象本身;
    // 如果进行下面的操作就相当于对MyStruct对象本身进行了修改
    *m = MyStruct {
        name: "new".to_string(),
    };
}

fn fn_ref_test2(m: &mut MyStruct) {
    m.name = "new2".to_string();
}

fn fn_ref_test3(m: &MyStruct) {
    // 这里会报错: `m` is a `&` reference, so the data it refers to cannot be written
    // m是一个不可变引用对象
    // *m = MyStruct {
    //     name: "new".to_string(),
    // };
    println!("name is: {:?}", m.name);
}

fn fn_ref_test4(m: &MyStruct) {
    // 即使改为 n fn_ref_test4(mut m: &MyStruct) 也会失败
    // 避免了Java中方法中的一个参数重载的问题
    // 这里会报错: temporary value is freed at the end of this statement
    // m = &mut MyStruct {
    //     name: "new3".to_string(),
    // };
    println!("name is: {:?}", m.name);
}

#[test]
fn test_ref_is_changed() {
    let mut m = MyStruct {
        name: "old".to_string(),
    };
    fn_ref_test(&mut m);
    assert_eq!(m.name, "new");

    fn_ref_test2(&mut m);
    assert_eq!(m.name, "new2");

    fn_ref_test3(&m);

    fn_ref_test4(&m);
}
