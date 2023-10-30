use std::ops::Add;
use md5;

pub fn part_1() -> i32 {
    let data = String::from("yzbqklnj");
    let mut i = 0;
    loop {
        let hash = md5::compute(data.clone().add(&i.to_string()));
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

pub fn part_2() -> i32 {
    let data = String::from("yzbqklnj");
    let mut i = 0;
    loop {
        let hash = md5::compute(data.clone().add(&i.to_string()));
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("000000") {
            return i;
        }
        i += 1;
    }
}
