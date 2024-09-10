// 开启全局允许未使用
#![allow(unused)]

use crate::prelude::*;

use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn read_path_dir() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry = String::try_from(W(&entry))?;
        println!("{entry:?}")
    }

    Ok(())
}

fn main() {
    read_path_dir();
    println!("Hello, world!");
}
