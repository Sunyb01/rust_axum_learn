enum ME1 {
    A,
    B(MyIfStruct0),
}

struct MyIfStruct0 {
    x: MyIfStruct1,
    y: i32,
}

struct MyIfStruct1 {
    z: i32,
    n: String,
}

fn if_let_struct(i: i32) -> ME1 {
    if 0 == i % 2 {
        ME1::B(MyIfStruct0 {
            x: MyIfStruct1 {
                z: 10,
                n: String::from("value"),
            },
            y: 20,
        })
    } else {
        ME1::A
    }
}

#[cfg(test)]
mod test {
    use super::if_let_struct;
    use super::ME1;
    use crate::{MyIfStruct0, MyIfStruct1};
    use rand::{thread_rng, Rng};
    #[test]
    fn test_if_let_result() {
        let mut rng = thread_rng();
        let m = if_let_struct(rng.gen());
        let z1 = if let ME1::B(MyIfStruct0 {
            x: MyIfStruct1 { z, .. },
            ..
        }) = m
        {
            z
        } else {
            0
        };
        println!("z = {:?}", z1);
    }
}
