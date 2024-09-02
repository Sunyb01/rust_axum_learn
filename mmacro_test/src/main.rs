use mmacro::*;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro, Debug)]
pub struct MyStruct12 {}

fn main() {
    MyStruct12::hello_macro();
}
