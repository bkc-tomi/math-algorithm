use std::io;

pub fn u8(msg: &str) -> u8 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: u8 = input.trim().parse().ok().unwrap();
    return a1;
}

pub fn u16(msg: &str) -> u16 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: u16 = input.trim().parse().ok().unwrap();
    return a1;
}

pub fn u32(msg: &str) -> u32 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: u32 = input.trim().parse().ok().unwrap();
    return a1;
}

pub fn i8(msg: &str) -> i8 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: i8 = input.trim().parse().ok().unwrap();
    return a1;
}

pub fn i16(msg: &str) -> i16 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: i16 = input.trim().parse().ok().unwrap();
    return a1;
}

pub fn i32(msg: &str) -> i32 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).ok();
    let a1: i32 = input.trim().parse().ok().unwrap();
    return a1;
}